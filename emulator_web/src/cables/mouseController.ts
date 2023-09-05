import { Grid } from "../grid";
import { CableId } from "./cable";
import { CableGraph } from "./cableGraph";
import { CableShape } from "./cableShape";
import Konva from "konva";

export class CableMouseController {
    graph: CableGraph
    cableLayer: Konva.Layer

    selectedCable: CableShape | null = null
    selectedPoint: number | null = null
    newCable: boolean

    constructor(graph: CableGraph, layer: Konva.Layer) {
        this.graph = graph
        this.cableLayer = layer
    }

    dragStart(pos: {x: number, y: number}) {
        let gridPos = this.graph.grid.worldToGrid(pos)

        if (this.selectedCable) {
            let point = this.selectedCable.findOrMakePoint(gridPos)
            if (point !== undefined) {
                this.selectedPoint = point
                this.newCable = false
            } else {
                this.selectedCable = null
                this.selectedPoint = null
            }
        } else {
            this.selectedCable = new CableShape(this.graph.grid)
            this.selectedCable.addPoint(gridPos)
            this.selectedCable.addPoint(gridPos)
            this.selectedPoint = 1
            this.cableLayer.add(this.selectedCable.shape)
            this.newCable = true
        }

    }
    
    drag(pos: { x: number, y: number }) {
        let gridPos = this.graph.grid.worldToGrid(pos)
        if (this.selectedCable !== null && this.selectedPoint !== null) {
            this.selectedPoint = this.selectedCable.movePointAligned(this.selectedPoint, gridPos)
        }
    }

    dragEnd(pos: {x: number, y: number}) {
        if (this.selectedCable) {
            this.selectedCable.removeFlatPoints()
            if (this.newCable) {
                let t = this.selectedCable
                this.selectedCable.shape.on('pointerdown', (() => {
                    this.selectedCable = t
                }).bind(this))
            }
            this.selectedCable = null
            this.selectedPoint = null
        }
    }
}