import { CableId } from './cables/cable'
import { SplitShape } from './cables/splitShape'

export class CacheEntry {
    split: SplitShape | undefined
    component: number | undefined
    cables: Array<CableId>

    constructor() {
        this.cables = []
    }

    insertCable(id: CableId) {
        let i = this.cables.findIndex((x) => x == id)
        if (i == -1) {
            this.cables.push(id)
        }
    }

    removeCable(id: CableId) {
        let i = this.cables.findIndex((x) => x == id)
        if (i != -1) {
            this.cables.splice(i,1)
        }
    }
}

export type CacheMap = {
    [x: number]: {
        [y: number]: CacheEntry
    }    
}

export class GridCache {
    cacheMap: CacheMap

    constructor() {
        this.cacheMap = {}
    }

    get(gridPos: [number,number]): CacheEntry {
        if(this.cacheMap[gridPos[0]] == undefined){
            this.cacheMap[gridPos[0]] = {}
        }
        if(this.cacheMap[gridPos[0]][gridPos[1]] == undefined) {
            this.cacheMap[gridPos[0]][gridPos[1]] = new CacheEntry
        }
        return this.cacheMap[gridPos[0]][gridPos[1]]
    }

    *getRange(from: [number,number], to: [number,number]): Iterable<CacheEntry> {
        let minmax = (a,b) => a < b ? [a, b] : [b, a]
        
        let [xMin,xMax] = minmax(from[0],to[0])
        for(let i = xMin; i <= xMax; i++) {
            let [yMin,yMax] = minmax(from[1],to[1])
            for(let j = yMin; j <= yMax; j++) {
                yield this.get([i,j])
            }
        }
    }

    insertCablePoints(id: CableId, points: Array<number>) {
        for(let i = 0; i < points.length - 1; i += 2) {
            for (let entry of this.getRange([points[i], points[i+1]],[points[i+2], points[i+3]])) {
                entry.insertCable(id)   
            }
        }
    }

    removeCablePoints(id: CableId, points: Array<number>) {
        for(let i = 0; i < points.length - 1; i += 2) {
            for (let entry of this.getRange([points[i], points[i+1]],[points[i+2], points[i+3]])) {
                entry.removeCable(id)   
            }
        }
    }
}
