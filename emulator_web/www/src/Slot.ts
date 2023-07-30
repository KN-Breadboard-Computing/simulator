import Konva from 'konva';
import { Cable } from './Cable';
import { GraphNode } from './GraphNode';
import { NodeId } from 'emulator';

export enum SlotType {
    INPUT,
    OUTPUT
}

export type SlotId = number

export class Slot extends Konva.Circle {
    node_id: NodeId
    slot_id: SlotId
    slotType: SlotType
    initialFill: string
    connection: Cable

    constructor(config: Konva.CircleConfig, node_id: NodeId, slot_id: SlotId) {
        super(config);
        this.slotType = config.slotType
        this.node_id = node_id
        this.slot_id = slot_id
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
}

export class InputSlot extends Slot {
    constructor(config: Konva.CircleConfig, node_id: NodeId, slot_id: SlotId) {
        super({ ...config, slotType: SlotType.INPUT }, node_id, slot_id);
    }
}

export enum OutputValue {
    ZERO,
    ONE,
    UNDEFINED
}

export class OutputSlot extends Slot {
    output: OutputValue;

    constructor(config: Konva.CircleConfig, node_id: NodeId, slot_id: SlotId) {
        super({ ...config, slotType: SlotType.OUTPUT }, node_id, slot_id);
    }

    setValue(value: OutputValue) {
        this.output = value;
        this.connection?.updateValue(value)
    }
}
