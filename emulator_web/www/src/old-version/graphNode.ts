import Konva from 'konva'
import { InputSlot, OutputSlot, Slot } from './slot'
import { Context } from './context'
import { NodeId } from 'emulator'

export interface GraphNodeConfig {
    id: NodeId
    x: number
    y: number
}

export class GraphNode {
    group: Konva.Group
    slotGroup: Konva.Group
    shapeGroup: Konva.Group
    id: NodeId

    onNodeUpdate?: (this: GraphNode) => void

    inputSlots: InputSlot[] = []
    outputSlots: OutputSlot[] = []

    constructor(config: GraphNodeConfig) {
        this.group = new Konva.Group({draggable: true})
        this.slotGroup = new Konva.Group()
        this.shapeGroup = new Konva.Group()
        this.group.add(this.shapeGroup)
        this.group.add(this.slotGroup)
        this.id = config.id
        this.group.setPosition({ x: config.x, y: config.y })
    }

    addSlot(slot: InputSlot | OutputSlot): void {
        if (slot instanceof InputSlot) {
            this.inputSlots.push(slot as InputSlot)
        } else {
            this.outputSlots.push(slot as OutputSlot)
        }
        this.slotGroup.add(slot)
    }

    updateNodeState() {
        if (this.onNodeUpdate != undefined) {
            this.onNodeUpdate()
        }
    }
}
