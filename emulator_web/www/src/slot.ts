import Konva from 'konva'
import { Cable } from './cable'
import { GraphNode } from './graphNode'
import { NodeId } from 'emulator'

export type SlotId = number

export enum SlotType {
    INPUT,
    OUTPUT
}

export class Slot extends Konva.Circle {
    nodeId: NodeId
    slotId: SlotId
    initialFill: string
    connection: Cable

    constructor(config: Konva.CircleConfig, nodeId: NodeId, slotId: SlotId) {
        super(config)
        this.nodeId = nodeId
        this.slotId = slotId
    }

    select() {
        this.initialFill = this.fill()
        this.fill('blue')
    }

    deselect() {
        this.fill(this.initialFill)
    }

    connect(cable: Cable) {
        this.connection = cable
    }

    public static getSlotType(slot: InputSlot | OutputSlot): SlotType {
        return slot instanceof InputSlot ? SlotType.INPUT : SlotType.OUTPUT
    }

    public static areSlotsCompatible(a: InputSlot | OutputSlot, b: InputSlot | OutputSlot) {
        if (this.getSlotType(a) !== this.getSlotType(b)) {
            return true
        }
        return false
    }
}

export class InputSlot extends Slot {
    constructor(config: Konva.CircleConfig, nodeId: NodeId, slotId: SlotId) {
        super({ ...config }, nodeId, slotId)
    }
}

export enum OutputValue {
    ZERO,
    ONE,
    UNDEFINED
}

export class OutputSlot extends Slot {
    output: OutputValue

    constructor(config: Konva.CircleConfig, nodeId: NodeId, slotId: SlotId) {
        super({ ...config, slotType: SlotType.OUTPUT }, nodeId, slotId)
    }

    setValue(value: OutputValue) {
        this.output = value
        this.connection?.updateValue(value)
    }
}
