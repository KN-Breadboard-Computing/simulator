import Konva from "konva"
import { Shape, ShapeConfig } from "konva/lib/Shape"

export interface GraphNodeShape {
    getShape(): Konva.Shape
    getInputSlotsPositions(count: number): Konva.Vector2d[]
    getOutputSlotsPositions(count: number): Konva.Vector2d[]
}

export class GraphNodeRectangleShape implements GraphNodeShape {
    gridWidth: number
    gridHeight: number

    maxInputSlotsCnt: number
    maxOutputSlotsCnt: number

    gridSpacing: number

    constructor(gridWidth: number, gridHeight: number, gridSpacing: number) {
        this.gridWidth = gridWidth
        this.gridHeight = gridHeight

        this.gridSpacing = gridSpacing

        this.maxInputSlotsCnt = gridWidth - 1
        this.maxOutputSlotsCnt = gridHeight - 1
    }

    getShape(): Konva.Shape {
        return new Konva.Rect({
            x: 0,
            y: 0,
            width: this.gridWidth * this.gridSpacing,
            height: this.gridHeight * this.gridSpacing,
            fill: 'white',
            stroke: 'black',
            strokeWidth: 4
        })
    }
    
    getInputSlotsPositions(count: number): Konva.Vector2d[] {
        return this.getSlotsPositionsHelper(count, 0)
    }

    getOutputSlotsPositions(count: number): Konva.Vector2d[] {
        return this.getSlotsPositionsHelper(count, this.gridWidth * this.gridSpacing)
    }

    private getSlotsPositionsHelper(count: number, x: number): Konva.Vector2d[] {
        if (count > this.maxInputSlotsCnt) {
            console.error("Can't add that many slots")
            return null
        }

        let slotPositions: Konva.Vector2d[] = [];
        for (let i = 0; i < count; i++) {
            let position = {x: x, y: (i + 1) * ((this.gridHeight * this.gridSpacing) / (count + 1))}
            slotPositions.push(position)
        }

        return slotPositions
    }
}