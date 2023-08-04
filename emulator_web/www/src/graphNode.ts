import Konva from 'konva'
import { InputSlot, OutputSlot, Slot, SlotType } from './slot'
import { Context } from './context'
import { NodeId } from 'emulator'

export interface GraphNodeConfig {
    nodeId: NodeId
    context: Context
    x: number
    y: number
}

export class GraphNode extends Konva.Group {
    context: Context
    nodeId: NodeId

    onNodeUpdate?: (this: GraphNode) => void

    inputSlots: InputSlot[] = []
    outputSlots: OutputSlot[] = []

    constructor(config: GraphNodeConfig) {
        super({
            draggable: true
        })

        this.context = config.context
        this.nodeId = config.nodeId
        this.setPosition({ x: config.x, y: config.y })
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
        if (this.onNodeUpdate != undefined) {
            this.onNodeUpdate()
        }
    }
}
