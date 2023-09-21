import Konva from 'konva'
import { Grid } from '../grid'
import { Cable, CableId } from './cable'
import { CableShape } from './cableShape'

export class CableGraph {
    cables: Cable[]
    cableShapes: CableShape[]
    grid: Grid

    constructor(grid: Grid) {
        this.cables = []
        this.cableShapes = []
        this.grid = grid
    }

    createCable(): CableId {
        let id = this.cables.length
        let cable = new Cable(id)
        this.cables.push(cable)
        return id
    }

    createCableWithShape(cableShape: CableShape): CableId {
        let id = this.createCable()
        this.cableShapes[id] = cableShape
        cableShape.id = id

        cableShape.shape.on('pointerdown', ((evt) => {
            let gridPos = this.grid.pointerGridPos()
            if (gridPos != undefined) {
                this.grid.controllerRegister.registerCableController({graph: this, selection: {cable: cableShape}, gridPos})
            }
            evt.cancelBubble = true
        }).bind(this))

        this.cableShapeMoved(cableShape, cableShape.controlPoints)
        return id
    }

    cableShapeMoved(cableShape: CableShape, oldPos?: Array<number>) {
        if (cableShape.id == undefined) {
            throw 'Gościu nie ma id'
        }
        if (oldPos != undefined) {
            this.grid.cache.removeCablePoints(cableShape.id, oldPos)
        }

        for (let end of [cableShape.first(), cableShape.last()]) {
            if (this.grid.cablesAmountAt(end) > 0) {
                this.grid.addSplit(end)
                for (let cableEntry of this.grid.cablesAt(end)) {
                    this.splitCable(cableEntry, end)
                }
            }
        }

        this.grid.cache.insertCablePoints(cableShape.id, cableShape.controlPoints)
    }

    splitCable(id: CableId, gridPos: [number, number]) {
        let first = this.cableShapes[id]

        if (
            (gridPos[0] == first.first()[0] && gridPos[1] == first.first()[1]) ||
            (gridPos[0] == first.last()[0] && gridPos[1] == first.last()[1])
        )
            return

        let i = first.findOrMakePoint(gridPos)
        if (i == undefined) throw 'coś ty dał do tej funkcji'
            
        let firstOld = first.controlPoints.slice()
        let otherPoints = [gridPos[0], gridPos[1], ...first.controlPoints.splice(2 * (i + 1), Infinity)]
        this.cableShapeMoved(first, firstOld)
        let other = new CableShape(this.grid)
        console.log(first, other, gridPos)
        other.controlPoints = otherPoints
        this.grid.cableLayer.add(other.shape)
        this.createCableWithShape(other)
        console.log('first', first)
        console.log('other', other)
    }
}
