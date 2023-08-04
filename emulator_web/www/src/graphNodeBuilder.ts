import Konva from 'konva'
import { GraphNode } from './graphNode'
import { NodeId } from 'emulator'
import { Context } from './context'
import { GraphNodeShape } from './graphNodeShape'
import { InputSlot, OutputSlot, SlotType } from './slot'

export interface GraphNodeBuilderConfig {
    nodeId: NodeId
    context: Context
    baseShape: GraphNodeShape
    x: number
    y: number
    scale: number
}

/*
componentInfo: ComponentInfo
context: Context
*/

export class GraphNodeBuilder {
    graphNode: GraphNode
    context: Context

    baseShape: GraphNodeShape

    constructor(config: GraphNodeBuilderConfig) {
        let baseShape = config.baseShape.getShape()
        this.graphNode = new GraphNode({
            nodeId: config.nodeId,
            baseShape: baseShape,
            context: config.context,
            x: config.x,
            y: config.y
        })

        let boundingBox = baseShape.getClientRect()
        this.graphNode.offset({
            x: boundingBox.width / 2,
            y: boundingBox.height / 2
        })

        this.baseShape = config.baseShape
    }

    public getGraphNode(): GraphNode {
        return this.graphNode
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
            padding: 20,
            align: 'center'
        })
        this.graphNode.add(label)
        return this
    }

    public setOnClick(func: () => void): GraphNodeBuilder {
        this.graphNode.on('click', func)
        return this
    }

    public setOnHover(func: () => void): GraphNodeBuilder {
        this.graphNode.on('mouseover', func)
        return this
    }

    public setOffHover(func: () => void): GraphNodeBuilder {
        this.graphNode.on('mouseout', func)
        return this
    }

    public addOnClick(func: () => void): GraphNodeBuilder {
        this.graphNode.on('click', func)
        return this
    }

    public addOutputSlots(count: number): GraphNodeBuilder {
        let slotPosition = this.baseShape.getOutputSlotsPositions(count)
        for (let i = 0; i < count; i++) {
            let slot = this.createSlot(i, slotPosition[i], 'green', SlotType.OUTPUT)
            this.graphNode.addSlot(slot, slot.slotType)
        }
        return this
    }

    public addInputSlots(count: number): GraphNodeBuilder {
        let slotPosition = this.baseShape.getInputSlotsPositions(count)
        for (let i = 0; i < count; i++) {
            let slot = this.createSlot(i, slotPosition[i], 'red', SlotType.INPUT)
            this.graphNode.addSlot(slot, slot.slotType)
        }
        return this
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
