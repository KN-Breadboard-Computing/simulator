import Konva from 'konva'
import { GraphNode, GraphNodeConfig } from './graphNode'
import { Cable } from './cable'
import { Context } from './context'
import { OutputValue, Slot, SlotType } from './slot'
import { selected, setupSidePanel as setupSidePanel } from './sidePanel'
import { Vector2d } from 'konva/lib/types'
import { Graph, NodeId } from 'emulator'
import { Grid } from './grid'
import { Stage } from 'konva/lib/Stage'
import { GraphNodeBuilder, GraphNodeBuilderConfig } from './graphNodeBuilder'
import { GraphNodeRectangleShape, GraphNodeTriangleShape } from './graphNodeShape'
import { ComponentMeta } from './componentMeta'
import { FileManager } from './fileManager'
import { StageContent } from './stageContent'
import { StateManager } from './stateManager'

export class App {
    componentLayer: Konva.Layer
    cableLayer: Konva.Layer
    gridLayer: Konva.Layer

    graph: Graph

    stageContent: StageContent
    grid: Grid

    nodes: GraphNode[]
    cables: Cable[]
    stage: Stage

    context: Context

    stateManager: StateManager

    fileLoader: FileManager

    constructor() {
        this.componentLayer = new Konva.Layer()
        this.cableLayer = new Konva.Layer()
        this.gridLayer = new Konva.Layer()
        this.stageContent = new StageContent()
        this.stateManager = new StateManager(this)
        this.fileLoader = new FileManager({
            loadCallback: this.setStage.bind(this),
            saveCallback: this.getStage.bind(this)
        })
    }

    async run() {
        setupSidePanel()

        const emulator = await import('emulator')

        this.graph = emulator.Graph.new()

        this.setStage(new StageContent())

        this.context = new Context({
            addCable: this.addCable.bind(this),
            updateCables: this.updateCables.bind(this),
            updateSelectedSlot: this.stateManager.updateSelectedSlot.bind(this.stateManager),
            fetchFn: this.fetchFn.bind(this),
            updateFn: this.updateFn.bind(this)
        })

        this.setupPopupMenu()
    }

    addNode(context: Context, pos: Vector2d, componentMeta: ComponentMeta) {
        let nodeId = this.graph.add_comp({ type: componentMeta.type })
        let graphNodeBuilder = new GraphNodeBuilder({
            nodeId: nodeId,
            x: pos.x,
            y: pos.y,
            context: context,
            gridSpacing: this.grid.spacing,
            type: componentMeta.type
        })
        graphNodeBuilder
            .setShape(componentMeta.shape)
            .setSnapToGrid(this.grid.getSnapToGridFunc().bind(this.grid))
            .setInputSlots(componentMeta.inputSize)
            .setOutputSlots(componentMeta.outputSize)

        for (const tag of componentMeta.tags) {
            tag.addToBuild(graphNodeBuilder)
        }

        let comp = graphNodeBuilder.getGraphNode()

        // clip to grid
        let boundingBox = comp.getClientRect()
        let topLeftX = boundingBox.x + boundingBox.width / 2
        let topLeftY = boundingBox.y + boundingBox.height / 2
        comp.position(this.grid.getSnapToGridFunc(Math.floor)({ x: topLeftX, y: topLeftY }))

        this.nodes.push(comp)
        this.componentLayer.add(comp)

        console.log('Added node', nodeId.id)
    }

    getStage(): StageContent {
        console.log("nodes: " + this.nodes)
        return new StageContent({
            nodes: this.nodes,
            cables: this.cables    
        })
    }

    setStage(stageContent: StageContent) {
        this.stage = new Konva.Stage({
            container: 'container',
            width: window.innerWidth,
            height: window.innerHeight
        })

        this.componentLayer = new Konva.Layer()
        this.cableLayer = new Konva.Layer()
        this.gridLayer = new Konva.Layer()

        this.stage.add(this.gridLayer)
        this.stage.add(this.cableLayer)
        this.stage.add(this.componentLayer)

        let app = this
        this.stage.on('pointerclick', function () {
            let pos = app.stage.getPointerPosition()!

            if (selected != null) {
                app.addNode(app.context, pos, selected.component)
            }
        })

        this.nodes = []

        this.grid = new Grid(this.gridLayer, 20)

        for (var comp of stageContent.nodes) {
            this.nodes.push(comp)
            this.componentLayer.add(comp)
        }
        this.cables = stageContent.cables
    }

    addCable(a: Slot, b: Slot) {
        if (!Slot.areSlotsCompatible(a, b)) {
            console.warn("Can't connect slots of the same type")
            return
        }
        let cable = new Cable(a, b)
        this.cables.push(cable)
        this.cableLayer.add(cable)

        let output: Slot
        let input: Slot

        if (a.slotType == SlotType.INPUT && b.slotType == SlotType.OUTPUT) {
            input = a
            output = b
        } else {
            input = b
            output = a
        }

        console.log('Added connection', output.nodeId, output.slotId, input.nodeId, input.slotId)

        this.graph.add_conn(output.nodeId, output.slotId, input.nodeId, input.slotId)
        this.propagate(output.nodeId)
    }

    updateCables() {
        for (var cable of this.cables) {
            cable.updatePosition()
        }
    }

    fetchFn(node_id: NodeId): any {
        return this.graph.get_comp(node_id)
    }

    updateFn(node_id: NodeId, state: { type: string }) {
        this.graph.set_comp(node_id, state)
        this.propagate(node_id)
    }

    propagate(node_id: NodeId) {
        this.graph.propagate(node_id)
        this.updateAllNodes()
    }

    updateAllNodes() {
        for (const node of this.nodes) {
            node.updateNodeState()

            let output_state = this.graph.output_state(node.nodeId)

            for (const output of node.outputSlots) {
                let bit = output_state & 1
                output_state = output_state >> 1
                output.setValue(bit == 1 ? OutputValue.ONE : OutputValue.ZERO)
                console.log('Updated output', node.nodeId, output.slotId, bit, bit == 1)
            }
        }
    }

    setupPopupMenu() {
        let app = this
        let currentPopupComponent = this.stateManager.currentPopupComponent
        var menuNode = document.getElementById('menu')!
        document.getElementById('rotate-button')?.addEventListener('click', () => {
            currentPopupComponent?.rotate(90)
        })

        document.getElementById('delete-button')?.addEventListener('click', () => {
            currentPopupComponent?.destroy()
        })

        window.addEventListener('click', () => {
            menuNode.style.display = 'none'
        })

        this.stage.on('contextmenu', function (e) {
            e.evt.preventDefault()
            if (e.target === app.stage) {
                return
            }
            currentPopupComponent = e.target.getParent()

            menuNode.style.display = 'initial'
            var containerRect = app.stage.container().getBoundingClientRect()
            menuNode.style.top = containerRect.top + app.stage.getPointerPosition()!.y + 4 + 'px'
            menuNode.style.left = containerRect.left + app.stage.getPointerPosition()!.x + 4 + 'px'
        })
    }
}
