export type CableId = number

export class Cable {
    id: CableId
    neighbours: Array<[number, number, CableId]>
    connectedComponents: Array<number>
    controlPoints: Array<[number, number]>

    constructor(id: number) {
        this.id = id
        this.neighbours = []
        this.connectedComponents = []
        this.controlPoints = []
    }

    addPoint(id: number, gridPos: [number, number]) {
        this.controlPoints.splice(id, 0, [gridPos[0], gridPos[1]])
    }
    
    movePoint(id: number, gridPos: [number, number]) {
        this.controlPoints[id] = gridPos
    }

    /* Przesuwa punkt, dodając albo przesuwając w razie czego nowe,
        żeby zachować równologłość kabla do osi siatki 
        tj. dla dwóch kolejnych punktów [x1,y1],[x2,y2] albo x1 == x2 albo y1 == y2 
        zwraca aktualne id punktu, w razie jakby doszło do dodania nowego*/
    movePointAligned(id: number, gridPos: [number, number]): number {
        this.controlPoints[id] = [gridPos[0],gridPos[1]]

        return this.alignToPoint(id)
    }

    alignToPoint(id: number) : number {
        for (const neigh of [-1, 1]) {
            let neighId = id + neigh
            if (neighId < 0 || neighId >= this.controlPoints.length) continue
            if (neighId == 0) {
                this.addPoint(1, this.controlPoints[0])
                neighId += 1
                id += 1
            }
            if (neighId == this.controlPoints.length - 1) {
                this.addPoint(this.controlPoints.length - 2, this.controlPoints[this.controlPoints.length - 1])
            }
            let neighNeighId = neighId + neigh
            let neighNeighPoint = this.controlPoints[neighNeighId]
            let neighPoint = this.controlPoints[neighId]
            let point = this.controlPoints[id]

            if(neighPoint[0] != point[0] && neighPoint[1] != point[1]) {
                if(neighPoint[0] == neighNeighPoint[0] && neighPoint[1] == neighNeighPoint[1]) {
                    let axis = Math.abs(point[0] - neighPoint[0]) > Math.abs(point[1] - neighPoint[1]) ? 0 : 1
                    neighPoint[axis] = point[axis]
                } else {
                    let axis = neighPoint[0] != neighNeighPoint[0] ? 0 : 1
                    neighPoint[axis] = point[axis]
                }
            }
        }
        return id
    }

    findControlPoint(gridPos: [number,number]) : number | undefined {
        let i = this.controlPoints.findIndex((point) => point[0] == gridPos[0] && point[1] == gridPos[1])
        if (i != -1) return i
    }

    pointToSegment(gridPos: [number, number]): number | undefined {
        function in_range(x: number, a: number, b: number): boolean {
            let [min, max] = a < b ? [a, b] : [b, a]
            return min <= x && x <= max
        }
        for (let i = 0; i < this.controlPoints.length - 1; i++) {
            let start = this.controlPoints[i]
            let end = this.controlPoints[i + 1]
            if (in_range(gridPos[0], start[0], end[0]) && in_range(gridPos[1], start[1], end[1])) {
                return i
            }
        }
    }
}
