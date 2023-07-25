import Konva from 'konva'
import { Slot } from './Slot'

export class Cable extends Konva.Line {
    parents: [Konva.Shape, Konva.Shape]
    line: Konva.Line

    update() {
        let parent1pos = this.parents[0].getAbsolutePosition()
        let parent2pos = this.parents[1].getAbsolutePosition()
        this.points([parent1pos.x, parent1pos.y, parent2pos.x, parent2pos.y])
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