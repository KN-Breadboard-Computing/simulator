import Konva from 'konva'
import { Cable, CableId } from './cable'
import { Grid } from '../grid'

export type CableLineEventListener = Konva.KonvaEventListener<Konva.Line, any>

export class CableDrawGroup {
    cableLines: Array<Konva.Line>
    grid: Grid
    group: Konva.Group

    constructor(grid: Grid) {
        this.group = new Konva.Group()
        this.grid = grid
        this.cableLines = []
    }

    addCableLine(cable: Cable) {
        let l = new Konva.Line({ stroke: 'black', strokeWidth: 5 })
        this.cableLines[cable.id] = l
        this.group.add(l)
        this.updateCableLine(cable)
    }

    updateCableLine(cable: Cable) {
        let l = this.cableLines[cable.id]
        l.points(cable.controlPoints.flatMap(([x,y]) => [x * this.grid.spacingX, y * this.grid.spacingY]))
    }

    onCableLine(cable: Cable, evtStr: string | number, handler: CableLineEventListener) {
        let l = this.cableLines[cable.id]
        l.on(evtStr,handler)
    }

    removeCableLine(cable: Cable) {
        let cableLine = this.cableLines[cable.id]
        cableLine.destroy()
    }
}
