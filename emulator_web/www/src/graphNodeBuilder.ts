import Konva from 'konva'
import { GraphNode } from './graphNode'
import { NodeId } from 'emulator'
import { Context } from './context'
import { GraphNodeShape } from './graphNodeShape'
import { InputSlot, OutputSlot, SlotType } from './slot'
import { NodeEventListener, Tag } from './componentMeta'

export interface GraphNodeBuilderConfig {
    nodeId: NodeId
    context: Context
    type: string
    x: number
    y: number
    gridSpacing: number
}

export class GraphNodeBuilder {
    graphNode: GraphNode
    context: Context
    baseShape: GraphNodeShape | undefined
    type: string
    gridSpacing: number

    constructor(config: GraphNodeBuilderConfig) {
        this.graphNode = new GraphNode({
            nodeId: config.nodeId,
            context: config.context,
            x: config.x,
            y: config.y
        })
        this.context = config.context
        this.gridSpacing = config.gridSpacing
        this.type = config.type
    }

    public getGraphNode(): GraphNode {
        return this.graphNode
    }

    public setShape(shape: GraphNodeShape): GraphNodeBuilder {
        this.baseShape = shape

        let graphicalShape = this.baseShape.getShape(this.gridSpacing)

        this.graphNode.width(graphicalShape.width())
        this.graphNode.height(graphicalShape.height())

        let boundingBox = graphicalShape.getClientRect()
        this.graphNode.offset({
            x: boundingBox.width / 2,
            y: boundingBox.height / 2
        })

        this.graphNode.add(graphicalShape)
        return this
    }

    public setSnapToGrid(func: (pos: Konva.Vector2d) => Konva.Vector2d): GraphNodeBuilder {
        this.graphNode.dragBoundFunc(func)
        return this
    }

    public setLabel(text: string): GraphNodeBuilder {
        let label = new Konva.Text({
            x: 0,
            y: 0,
            text: text,
            fontSize: 18,
            fontFamily: 'Calibri',
            fill: 'black',
            width: this.graphNode.width(),
            height: this.graphNode.height(),
            align: 'center',
            verticalAlign: 'middle',
            id: 'mainLabel'
        })
        this.graphNode.add(label)
        return this
    }

    public setOnClick<T>(func: NodeEventListener<T>): GraphNodeBuilder {
        this.graphNode.on('click', this.bindEventListener(func))
        return this
    }

    public setOnHover<T>(func: NodeEventListener<T>): GraphNodeBuilder {
        this.graphNode.on('mouseover', this.bindEventListener(func))
        return this
    }

    public setOffHover<T>(func: NodeEventListener<T>): GraphNodeBuilder {
        this.graphNode.on('mouseout', this.bindEventListener(func))
        return this
    }

    public setOnNodeUpdate<T>(func: NodeEventListener<T>): GraphNodeBuilder {
        this.graphNode.onNodeUpdate = this.bindEventListener(func)
        return this
    }

    public setOutputSlots(count: number): GraphNodeBuilder {
        if (this.baseShape != undefined) {
            let slotPosition = this.baseShape.getOutputSlotsPositions(count, this.gridSpacing)
            for (let i = 0; i < count; i++) {
                let slot = this.createSlot(i, slotPosition[i], 'green', SlotType.OUTPUT)
                this.graphNode.addSlot(slot)
            }
        } else {
            console.warn('Tried to add slots without adding shape first')
        }
        return this
    }

    public setInputSlots(count: number): GraphNodeBuilder {
        if (this.baseShape != undefined) {
            let slotPosition = this.baseShape.getInputSlotsPositions(count, this.gridSpacing)
            for (let i = 0; i < count; i++) {
                let slot = this.createSlot(i, slotPosition[i], 'red', SlotType.INPUT)
                this.graphNode.addSlot(slot)
            }
        } else {
            console.warn('Tried to add slots without adding shape first')
        }
        return this
    }

    public addTag(tag: Tag): GraphNodeBuilder {
        tag.addToBuild(this)
        return this
    }

    private bindEventListener<T>(listener: NodeEventListener<T>): (this: GraphNode) => void {
        let func = () => {
            listener.bind(this.graphNode)(
                () => {
                    return this.context.fetchFn(this.graphNode.nodeId)
                },
                (state: T) => {
                    this.context.updateFn(this.graphNode.nodeId, { type: this.type, ...state })
                }
            )
        }
        return func
    }

    private createSlot(i: number, pos: Konva.Vector2d, color: string, type: SlotType): InputSlot | OutputSlot {
        const config = {
            x: pos.x,
            y: pos.y,
            radius: 5,
            fill: color,
            stroke: 'black',
            strokeWidth: 2
        }
        let slot: InputSlot | OutputSlot

        if (type === SlotType.OUTPUT) {
            slot = new OutputSlot(config, this.graphNode.nodeId, i)
        } else {
            slot = new InputSlot(config, this.graphNode.nodeId, i)
        }

        slot.on('click', () => this.graphNode.context.updateSelectedSlot(slot))
        return slot
    }
}
