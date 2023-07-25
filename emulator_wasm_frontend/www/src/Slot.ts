import Konva from 'konva';

export enum SlotType {
    INPUT,
    OUTPUT
}

export class Slot extends Konva.Circle {
    slotType: SlotType;
    initialFill: string

    constructor(config: Konva.CircleConfig & { slotType: SlotType }) {
        super(config);
        this.slotType = config.slotType;
    }

    select() {
        this.initialFill = this.fill()
        this.fill('blue')
    }

    deselect() {
        this.fill(this.initialFill)
    }
}