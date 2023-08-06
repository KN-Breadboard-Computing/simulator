import Konva from 'konva'
import { Shape, ShapeConfig } from 'konva/lib/Shape'

export interface GraphNodeShape {
    getShape(gridSpacing: number): Konva.Shape
    getInputSlotsPositions(count: number, gridSpacing: number): Konva.Vector2d[]
    getOutputSlotsPositions(count: number, gridSpacing: number): Konva.Vector2d[]
}

export class GraphNodeRectangleShape implements GraphNodeShape {
    gridWidth: number
    gridHeight: number

    maxInputSlotsCnt: number
    maxOutputSlotsCnt: number

    constructor(gridWidth: number, gridHeight: number) {
        this.gridWidth = gridWidth
        this.gridHeight = gridHeight

        this.maxInputSlotsCnt = gridHeight - 1
        this.maxOutputSlotsCnt = gridHeight - 1
    }

    getShape(gridSpacing: number): Konva.Shape {
        return new Konva.Rect({
            x: 0,
            y: 0,
            width: this.gridWidth * gridSpacing,
            height: this.gridHeight * gridSpacing,
            fill: 'white',
            stroke: 'black',
            strokeWidth: 4
        })
    }

    getInputSlotsPositions(count: number, gridSpacing: number): Konva.Vector2d[] {
        if (count > this.maxInputSlotsCnt) {
            console.error("Can't add that many slots")
            count = this.maxInputSlotsCnt
        }
        return this.getSlotsPositionsHelper(count, 0, gridSpacing)
    }

    getOutputSlotsPositions(count: number, gridSpacing: number): Konva.Vector2d[] {
        if (count > this.maxOutputSlotsCnt) {
            console.error("Can't add that many slots")
            count = this.maxOutputSlotsCnt
        }
        return this.getSlotsPositionsHelper(count, this.gridWidth * gridSpacing, gridSpacing)
    }

    private getSlotsPositionsHelper(count: number, x: number, gridSpacing: number): Konva.Vector2d[] {
        let slotPositions: Konva.Vector2d[] = []
        for (let i = 0; i < count; i++) {
            let position = { x: x, y: (i + 1) * ((this.gridHeight * gridSpacing) / (count + 1)) }
            slotPositions.push(position)
        }

        return slotPositions
    }
}

export class GraphNodeTriangleShape implements GraphNodeShape {
    gridWidth: number
    gridHeight: number

    maxInputSlotsCnt: number
    maxOutputSlotsCnt: number = 1

    constructor(gridWidth: number, gridHeight: number) {
        this.gridWidth = gridWidth
        this.gridHeight = gridHeight

        this.maxInputSlotsCnt = gridWidth - 1
    }

    getShape(gridSpacing: number): Konva.Shape {
        let point0 = { x: 0, y: 0 }
        let point1 = { x: 0, y: this.gridHeight * gridSpacing }
        let point2 = { x: this.gridWidth * gridSpacing, y: (this.gridHeight * gridSpacing) / 2 }
        return new Konva.Line({
            points: [point0.x, point0.y, point1.x, point1.y, point2.x, point2.y],
            closed: true,
            fill: 'white',
            stroke: 'black',
            strokeWidth: 4
        })
    }

    getInputSlotsPositions(count: number, gridSpacing: number): Konva.Vector2d[] {
        if (count > this.maxInputSlotsCnt) {
            console.error("Can't add that many slots")
            count = this.maxInputSlotsCnt
        }

        let slotPositions: Konva.Vector2d[] = []

        for (let i = 0; i < count; i++) {
            let position = { x: 0, y: (i + 1) * ((this.gridHeight * gridSpacing) / (count + 1)) }
            slotPositions.push(position)
        }

        return slotPositions
    }

    getOutputSlotsPositions(count: number, gridSpacing: number): Konva.Vector2d[] {
        if (count > this.maxOutputSlotsCnt) {
            console.error("Can't add that many slots")
            count = this.maxOutputSlotsCnt
        }

        return [{ x: this.gridWidth * gridSpacing, y: (this.gridHeight * gridSpacing) / 2 }]
    }
}
