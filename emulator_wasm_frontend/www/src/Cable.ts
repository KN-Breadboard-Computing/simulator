import Konva from 'konva';
import { OutputValue, Slot } from './Slot';

export class Cable extends Konva.Line {
    parents: [Slot, Slot];

    constructor(parent1: Slot, parent2: Slot) {
        const parent1pos = parent1.getAbsolutePosition();
        const parent2pos = parent2.getAbsolutePosition();
        super({
            points: [parent1pos.x, parent1pos.y, parent2pos.x, parent2pos.y],
            stroke: 'black'
        });

        this.parents = [parent1, parent2];
        parent1.connect(this);
        parent2.connect(this);
    }

    updatePosition(): void {
        const [parent1, parent2] = this.parents.map(parent => parent.getAbsolutePosition());
        this.points([parent1.x, parent1.y, parent2.x, parent2.y]);
    }

    updateValue(value: OutputValue): void {
        this.stroke(value === OutputValue.ONE ? 'green' : 'black');
    }
}
