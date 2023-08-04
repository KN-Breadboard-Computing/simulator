import Konva from 'konva'
import { InputSlot, OutputSlot, Slot, SlotType } from './slot'
import { Context } from './context'
import { NodeId } from 'emulator'

export interface GraphNodeConfig {
    nodeId: NodeId
    baseShape: Konva.Shape
    context: Context
    x: number
    y: number
}

export class GraphNode extends Konva.Group {
    context: Context

    nodeId: NodeId

    inputSlots: InputSlot[] = []
    outputSlots: OutputSlot[] = []

    constructor(config: GraphNodeConfig) {
        super({
            draggable: true
        })

        this.context = config.context

        this.nodeId = config.nodeId
        //this.componentInfo = config.componentInfo

        this.setPosition({ x: config.x, y: config.y })

        this.width(config.baseShape.width())
        this.height(config.baseShape.height())

        this.add(config.baseShape)
        // this.componentInfo.onStart(
        //     () => {
        //         return this.context.fetchFn(this.nodeId)
        //     },
        //     a => {
        //         this.context.updateFn(this.nodeId, { type: this.componentInfo.type, ...a })
        //     },
        //     { group: this }
        // )

        this.on('dragmove', () => this.context.updateCables())
    }

    addSlot(slot: Slot, slotType: SlotType): void {
        if (slotType === SlotType.INPUT) {
            this.inputSlots.push(slot as InputSlot)
        } else {
            this.inputSlots.push(slot as OutputSlot)
        }
        this.add(slot)
    }

    updateNodeState() {
        console.log('aaa')
        // this.componentInfo.onUpdate(
        //     () => {
        //         return this.context.fetchFn(this.nodeId)
        //     },
        //     a => {
        //         this.context.updateFn(this.nodeId, { type: this.componentInfo.type, ...a })
        //     },
        //     { group: this }
        // )
    }
}
