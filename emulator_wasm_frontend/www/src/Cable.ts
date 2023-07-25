import Konva from 'konva'
import { OutputValue, Slot } from './Slot'

export class Cable extends Konva.Line {
    parents: [Konva.Shape, Konva.Shape]
    line: Konva.Line

    updatePosition() {
        let parent1pos = this.parents[0].getAbsolutePosition()
        let parent2pos = this.parents[1].getAbsolutePosition()
        this.points([parent1pos.x, parent1pos.y, parent2pos.x, parent2pos.y])
    }

    updateValue(value: OutputValue) {
        if(value == OutputValue.ONE) {
            this.fill("green")
        }
        if(value == OutputValue.ZERO || value == OutputValue.UNDEFINED) {
            this.fill("black")
        }
    }

    constructor(parent1: Slot, parent2: Slot) {
        let parent1pos = parent1.getAbsolutePosition()
        let parent2pos = parent2.getAbsolutePosition()
        super({
            points: [parent1pos.x, parent1pos.y, parent2pos.x, parent2pos.y],
            stroke: 'black'
        })
        this.parents = [parent1, parent2];
        parent1.connect(this)
        parent2.connect(this)
    }
}