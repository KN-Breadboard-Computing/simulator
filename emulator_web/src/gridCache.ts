import { CableId } from './cables/cable'

export type CableCacheEntry = {
    type: 'cable'
    id: CableId
}

export type ComponentCacheEntry = {
    type: 'comp'
    //Placeholder
    id: number
}

export type CacheEntry = CableCacheEntry | ComponentCacheEntry

function areCacheEntryEqual(a: CacheEntry, b: CacheEntry) : boolean {
    return a.type == b.type && a.id == b.id
}

export type CacheMap = {
    [x: number]: {
        [y: number]: Array<CacheEntry>
    }    
}

export class GridCache {
    cacheMap: CacheMap

    constructor() {
        this.cacheMap = {}
    }

    get(gridPos: [number,number]): Array<CacheEntry> {
        if(this.cacheMap[gridPos[0]] == undefined){
            this.cacheMap[gridPos[0]] = {}
        }
        if(this.cacheMap[gridPos[0]][gridPos[1]] == undefined) {
            this.cacheMap[gridPos[0]][gridPos[1]] = []
        }
        return this.cacheMap[gridPos[0]][gridPos[1]]
    }

    insert(gridPos: [number, number], entry: CacheEntry) {
        let t = this.get(gridPos)
        let i = t.findIndex((x) => areCacheEntryEqual(x,entry))
        if (i == -1) {
            t.push(entry)
        }
    }

    remove(gridPos: [number,number], entry: CacheEntry) {
        let t = this.get(gridPos)
        let i = t.findIndex((x) => areCacheEntryEqual(x,entry))
        if (i != -1) {
            t.splice(i,1)
        }
    }

    insertRange(from: [number,number], to: [number,number], entry: CacheEntry) {
        let minmax = (a,b) => a < b ? [a, b] : [b, a]
        
        let [xMin,xMax] = minmax(from[0],to[0])
        for(let i = xMin; i <= xMax; i++) {
            let [yMin,yMax] = minmax(from[1],to[1])
            for(let j = yMin; j <= yMax; j++) {
                this.insert([i,j],entry)
            }
        }
    }

    removeRange(from: [number,number], to: [number,number], entry: CacheEntry) {
        let minmax = (a,b) => a < b ? [a, b] : [b, a]
        
        let [xMin,xMax] = minmax(from[0],to[0])
        for(let i = xMin; i <= xMax; i++) {
            let [yMin,yMax] = minmax(from[1],to[1])
            for(let j = yMin; j <= yMax; j++) {
                this.remove([i,j],entry)
            }
        }
    }

    insertCablePoints(id: CableId, points: Array<number>) {
        let entry = {type: 'cable' as const, id: id}
        for(let i = 0; i < points.length - 1; i += 2) {
            this.insertRange([points[i], points[i+1]],[points[i+2], points[i+3]],entry)
        }
    }

    removeCablePoints(id: CableId, points: Array<number>) {
        let entry = {type: 'cable' as const, id: id}
        for(let i = 0; i < points.length - 1; i += 2) {
            this.removeRange([points[i], points[i+1]],[points[i+2], points[i+3]],entry)
        }
    }
}
