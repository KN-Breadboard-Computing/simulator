import Konva from 'konva';

export enum SlotType {
    INPUT,
    OUTPUT
}

export class Slot extends Konva.Circle {
    slotType: SlotType;

    constructor(config: Konva.CircleConfig & { slotType: SlotType }) {
        super(config);
        this.slotType = config.slotType;
    }

    select() {
        console.log("selected")
        this.fill('blue')
    }

    deselect() {
        this.fill('black')
    }
}