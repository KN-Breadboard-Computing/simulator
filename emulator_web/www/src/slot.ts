import Konva from 'konva'
import { Cable } from './cable'
import { GraphNode } from './graphNode'
import { NodeId } from 'emulator'

export enum SlotType {
    INPUT,
    OUTPUT
}

export type SlotId = number

export class Slot extends Konva.Circle {
    nodeId: NodeId
    slotId: SlotId
    slotType: SlotType
    initialFill: string
    connection: Cable

    constructor(config: Konva.CircleConfig, nodeId: NodeId, slotId: SlotId) {
        super(config)
        this.slotType = config.slotType
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

    public static areSlotsCompatible(a: Slot, b: Slot) {
        if (a.slotType == b.slotType) {
            return false
        }
        return true
    }
}

export class InputSlot extends Slot {
    constructor(config: Konva.CircleConfig, nodeId: NodeId, slotId: SlotId) {
        super({ ...config, slotType: SlotType.INPUT }, nodeId, slotId)
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
