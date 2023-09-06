import Konva from 'konva'
import { GridCache } from './gridCache'

export class Grid {
    spacingX: number
    spacingY: number
    cache: GridCache
    cableLayer: Konva.Layer
    splitLayer: Konva.Layer

    constructor(spacingX: number, spacingY: number) {
        this.spacingX = spacingX
        this.spacingY = spacingY
        this.cache = new GridCache()
    }

    worldToGrid(pos: { x: number; y: number }): [number, number] {
        return [Math.round(pos.x / this.spacingX), Math.round(pos.y / this.spacingY)]
    }

    gridToWorld(gridPos: [number,number]) : [number,number] {
        return [gridPos[0] * this.spacingX, gridPos[1] * this.spacingY]
    }
}
