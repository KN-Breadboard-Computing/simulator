import Konva from 'konva';
import { Cable } from './Cable';

export enum SlotType {
    INPUT,
    OUTPUT
}

export class Slot extends Konva.Circle {
    slotType: SlotType
    initialFill: string
    connection: Cable

    constructor(config: Konva.CircleConfig) {
        super(config);
        this.slotType = config.slotType
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
    constructor(config: Konva.CircleConfig) {
        super({ ...config, slotType: SlotType.INPUT });
    }
}

export enum OutputValue {
    ZERO,
    ONE,
    UNDEFINED
}

export class OutputSlot extends Slot {
    output: OutputValue;

    constructor(config: Konva.CircleConfig) {
        super({ ...config, slotType: SlotType.OUTPUT });
    }

    setValue(value: OutputValue) {
        this.output = value;
        this.connection?.updateValue(value)
    }
}
