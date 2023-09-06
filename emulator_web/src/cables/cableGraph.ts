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
            if (this.grid.cache.get(end).length != 0) {
                let worldPos = this.grid.gridToWorld(end)
                let x = new Konva.Circle({
                    x: worldPos[0],
                    y: worldPos[1],
                    radius: 8,
                    stroke: 'black',
                    fill: 'gray',
                    strokeWidth: 5
                })
                this.grid.splitLayer.add(x)
                for (let cableEntry of this.grid.cache.get(end).filter(x => x.type == 'cable')) {
                    this.splitCable(cableEntry.id, end)
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
