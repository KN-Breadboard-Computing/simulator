import Konva from 'konva'

export class Grid {
    spacingX: number
    spacingY: number

    constructor(spacingX: number, spacingY: number) {
        this.spacingX = spacingX
        this.spacingY = spacingY
    }

    worldToGrid(pos: { x: number; y: number }): [number, number] {
        return [Math.round(pos.x / this.spacingX), Math.round(pos.y / this.spacingY)]
    }
}
