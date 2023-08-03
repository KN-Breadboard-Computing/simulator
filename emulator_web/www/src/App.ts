import Konva from 'konva';
import { GraphNode, GraphNodeConfig } from './GraphNode';
import { Cable } from './Cable';
import { Context } from './Context';
import { OutputValue, Slot, SlotType } from './Slot';
import { components } from './component_list';
import { selected, setup_side_panel } from './side_panel';
import { Vector2d } from 'konva/lib/types';
import { Graph, NodeId } from 'emulator';
import { Grid } from './Grid';
import { Stage } from 'konva/lib/Stage';

export class App {
    componentLayer: Konva.Layer;
    cableLayer: Konva.Layer;
    gridLayer: Konva.Layer;

    grid: Grid;
    graph: Graph
    nodes: GraphNode[]
    cables: Cable[]

    selectedSlot: Slot | null
    currentPopupComponent: GraphNode | null

    stage: Stage

    constructor() {
        this.componentLayer = new Konva.Layer();
        this.cableLayer = new Konva.Layer();
        this.gridLayer = new Konva.Layer();
        this.cables = []
        this.nodes = []
        this.selectedSlot = null
        this.stage = new Konva.Stage({
            container: 'container',
            width: window.innerWidth,
            height: window.innerHeight,
        });
    }

    async run() {
        setup_side_panel()

        const emulator = await import('emulator');

        this.graph = emulator.Graph.new()

        this.stage.add(this.gridLayer)
        this.stage.add(this.cableLayer);
        this.stage.add(this.componentLayer);

        this.grid = new Grid(this.gridLayer, 20)

        let context: Context = new Context({
            addCable: this.addCable.bind(this),
            updateCables: this.updateCables.bind(this),
            updateSelectedSlot: this.updateSelectedSlot.bind(this),
            fetchFn: this.fetchFn.bind(this),
            updateFn: this.updateFn.bind(this),
        })
    
        let app = this
        this.stage.on("pointerclick", function () {
            let pos = app.stage.getPointerPosition()

            if (selected != null) {
                app.addNode({
                    node_id: app.graph.add_comp({"type" : selected.component.type}),
                    x: pos.x,
                    y: pos.y,
                    componentInfo: selected.component,
                    context: context,
                    snapToGrid: app.grid.getSnapToGridFunc().bind(app.grid)
                })
            }
        })

        this.setupPopupMenu()
    }

    addNode(config: GraphNodeConfig) {
        let comp = new GraphNode(config)

        var boundingBox = comp.getClientRect()

        let topLeftX = boundingBox.x + boundingBox.width / 2;
        let topLeftY = boundingBox.y + boundingBox.height / 2;

        comp.position(this.grid.getSnapToGridFunc(Math.floor)({ x: topLeftX, y: topLeftY }))
        this.nodes.push(comp)
        this.componentLayer.add(comp)

        console.log("Added node", config.node_id)
    }

    addCable(a: Slot, b: Slot) {
        if(!this.areSlotsCompatible(a,b)) {
            // console.error("Can't connect slots of the same type");
            return;
        }
        let cable = new Cable(a,b);
        this.cables.push(cable)
        this.cableLayer.add(cable)

        if (a.slotType == SlotType.INPUT && b.slotType == SlotType.OUTPUT) {
            var input = a
            var output = b
        } else if (a.slotType == SlotType.OUTPUT && b.slotType == SlotType.INPUT) {
            var input = b
            var output = a
        }

        console.log("Added connection", output.node_id, output.slot_id, input.node_id, input.slot_id)

        this.graph.add_conn(output.node_id, output.slot_id, input.node_id, input.slot_id)
        this.propagate(output.node_id)
    }

    areSlotsCompatible(a: Slot, b: Slot) {
        if(a.slotType == b.slotType) {
            return false;
        }
        return true;
    }

    updateCables() {
        for(var cable of this.cables) {
            cable.updatePosition()
        }
    }

    updateSelectedSlot(clickedSlot: Slot) {
        if (this.selectedSlot == null) {
            this.selectSlot(clickedSlot)
        } else if (this.selectedSlot != clickedSlot) {
            if (!this.areSlotsCompatible(clickedSlot, this.selectedSlot)) {
                this.selectSlot(clickedSlot)
                return;
            }
            this.addCable(this.selectedSlot, clickedSlot)
            this.selectSlot(null)
        } else {
            this.selectSlot(null)
        }
    }

    fetchFn(node_id: NodeId) : any {
        return this.graph.get_comp(node_id)
    }

    updateFn(node_id: NodeId, state: { type: string }) {
        this.graph.set_comp(node_id, state)
        this.propagate(node_id)
    }

    selectSlot(slot: Slot) {
        this.selectedSlot?.deselect();
        this.selectedSlot = slot;
        slot?.select();
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
                console.log("Updated output", node.nodeId, output.slot_id, bit, bit == 1)
            }
        }
    }

    setupPopupMenu() {
        let app = this
        var menuNode = document.getElementById('menu');
        document.getElementById('rotate-button').addEventListener('click', () => {
            this.currentPopupComponent.rotate(90)
        });

        document.getElementById('delete-button').addEventListener('click', () => {
            this.currentPopupComponent.destroy()
          });
    
        window.addEventListener('click', () => {
            menuNode.style.display = 'none';
        });

        this.stage.on('contextmenu', function (e) {
            e.evt.preventDefault();
            if (e.target === app.stage) {
              return;
            }
            app.currentPopupComponent = e.target.getParent();

            menuNode.style.display = 'initial';
            var containerRect = app.stage.container().getBoundingClientRect();
            menuNode.style.top =
              containerRect.top + app.stage.getPointerPosition().y + 4 + 'px';
            menuNode.style.left =
              containerRect.left + app.stage.getPointerPosition().x + 4 + 'px';
          });
    }
}