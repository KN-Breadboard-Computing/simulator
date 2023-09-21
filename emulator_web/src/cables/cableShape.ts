import Konva from 'konva'
import { Grid } from '../grid'
import { CableId } from './cable'

export class CableShape {
    controlPoints: Array<number>
    id: CableId | undefined
    shape: Konva.Shape
    grid: Grid

    constructor(grid: Grid) {
        this.grid = grid
        this.controlPoints = []
        this.shape = new Konva.Shape({
            sceneFunc: this._sceneFunc.bind(this),
            stroke: 'black',
            strokeWidth: 5
        })
    }

    addPoint(gridPos: [number, number], id?: number) {
        if (id) {
            this.controlPoints.splice(2 * id, 0, gridPos[0], gridPos[1])
        } else {
            this.controlPoints.push(gridPos[0], gridPos[1])
        }
    }

    removePoint(id: number) {
        this.controlPoints.splice(2 * id, 2)
    }

    setPoint(id: number, gridPos: [number, number]) {
        this.controlPoints[2 * id] = gridPos[0]
        this.controlPoints[2 * id + 1] = gridPos[1]
    }

    getPoint(id: number): [number, number] {
        return [this.controlPoints[2 * id], this.controlPoints[2 * id + 1]]
    }

    pointsLength() : number {
        return this.controlPoints.length / 2
    }

    first(): [number,number] {
        return this.getPoint(0)
    }

    last(): [number,number] {
        return this.getPoint(this.pointsLength() - 1)
    }

    /* Przesuwa punkt, dodając albo przesuwając w razie czego nowe,
        żeby zachować równologłość kabla do osi siatki 
        tj. dla dwóch kolejnych punktów [x1,y1],[x2,y2] albo x1 == x2 albo y1 == y2 
        zwraca aktualne id punktu, w razie jakby doszło do dodania nowego*/
    movePointAligned(id: number, gridPos: [number, number]): number {
        this.setPoint(id, gridPos)

        return this.alignToPoint(id)
    }

    alignToPoint(id: number): number {
        for (const neigh of [-1, 1]) {
            let neighId = id + neigh
            if (neighId < 0 || neighId >= this.pointsLength()) continue
            if (neighId == 0) {
                this.addPoint([this.controlPoints[0], this.controlPoints[1]], 1)
                neighId += 1
                id += 1
            }
            if (neighId == this.pointsLength() - 1) {
                this.addPoint(
                    [
                        this.controlPoints[this.controlPoints.length - 2],
                        this.controlPoints[this.controlPoints.length - 1]
                    ],
                    this.controlPoints.length / 2 - 2
                )
            }
            let neighNeighId = neighId + neigh
            let neighNeighPoint = this.getPoint(neighNeighId)
            let neighPoint = this.getPoint(neighId)
            let point = this.getPoint(id)

            if (neighPoint[0] != point[0] && neighPoint[1] != point[1]) {
                if (neighPoint[0] == neighNeighPoint[0] && neighPoint[1] == neighNeighPoint[1]) {
                    let axis = Math.abs(point[0] - neighPoint[0]) > Math.abs(point[1] - neighPoint[1]) ? 0 : 1
                    neighPoint[axis] = point[axis]
                } else {
                    let axis = neighPoint[0] != neighNeighPoint[0] ? 0 : 1
                    neighPoint[axis] = point[axis]
                }
                this.setPoint(neighId, neighPoint)
            }
        }

        return id
    }

    removeFlatPoints() {
        for (let i = this.pointsLength() - 2; i >= 1; i--) {
            let [a, b] = [this.getPoint(i - 1), this.getPoint(i + 1)]
            for (const axis of [0, 1]) {
                if (a[axis] == b[axis]) {
                    this.removePoint(i)
                    break
                }
            }
        }
    }

    findControlPoint(gridPos: [number, number]): number | undefined {
        for(let i = 0; i < this.pointsLength(); i++) {
            let p = this.getPoint(i)
            if (p[0] == gridPos[0] && p[1] == gridPos[1]) {
                return i
            }
        }
    }

    pointToSegment(gridPos: [number, number]): number | undefined {
        function in_range(x: number, a: number, b: number): boolean {
            let [min, max] = a < b ? [a, b] : [b, a]
            return min <= x && x <= max
        }
        for (let i = 0; i < this.pointsLength() - 1; i++) {
            let start = this.getPoint(i)
            let end = this.getPoint(i+1)
            if (in_range(gridPos[0], start[0], end[0]) && in_range(gridPos[1], start[1], end[1])) {
                return i
            }
        }
    }

    findOrMakePoint(gridPos: [number, number]): number | undefined {
        let point = this.findControlPoint(gridPos)
        if (point === undefined) {
            let segment = this.pointToSegment(gridPos)
            if (segment !== undefined) {
                this.addPoint(gridPos, segment + 1)
                point = segment + 1
            }
        }
        return point
    }

    _sceneFunc(ctx: Konva.Context, shape: Konva.Shape) {
        if (this.controlPoints.length == 0) return
        ctx.beginPath()
        let start = this.grid.gridToWorld(this.getPoint(0))
        ctx.moveTo(start.x, start.y)
        for (let i = 1; i < this.controlPoints.length; i++) {
            let point = this.grid.gridToWorld(this.getPoint(i))
            ctx.lineTo(point.x, point.y)
        }
        ctx.fillStrokeShape(shape)
    }
}
