import { Grid } from "../grid";
import { CableId } from "./cable";
import { CableGraph } from "./cableGraph";
import { CableShape } from "./cableShape";
import Konva from "konva";

export class CableMouseController {
    graph: CableGraph

    selectedCable: CableShape | null = null
    selectedPoint: number | null = null
    newCable: boolean
    oldCablePos: Array<number> | undefined

    constructor(graph: CableGraph) {
        this.graph = graph
    }

    dragStart(pos: {x: number, y: number}) {
        let gridPos = this.graph.grid.worldToGrid(pos)

        if (this.selectedCable) {
            let point = this.selectedCable.findOrMakePoint(gridPos)
            if (point !== undefined) {
                this.selectedPoint = point
                this.newCable = false
                this.oldCablePos = this.selectedCable.controlPoints.slice()
                return
            }
        }

        this.selectedCable = new CableShape(this.graph.grid)
        this.selectedCable.addPoint(gridPos)
        this.selectedCable.addPoint(gridPos)
        this.selectedPoint = 1
        this.graph.grid.cableLayer.add(this.selectedCable.shape)
        this.newCable = true
        this.oldCablePos = undefined

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
                this.graph.createCableWithShape(this.selectedCable)
            } else {
                this.graph.cableShapeMoved(this.selectedCable, this.oldCablePos)
            }
            console.log(this.selectedCable)
            console.log(this.graph.grid.cache)
            console.log(this.graph)
            this.oldCablePos = undefined
            this.selectedCable = null
            this.selectedPoint = null
        }
    }
}