import Konva  from "konva";
import { Grid } from "../grid";
import { Cable, CableId } from "./cable";
import { CableShape } from "./cableShape";

export class CableGraph {
    cables : Cable[]
    cableShapes: CableShape[]
    grid: Grid
    drawGroup: Konva.Group

    constructor(grid: Grid) {
        this.cables = []
        this.cableShapes = []
        this.grid = grid
        this.drawGroup = new Konva.Group
    }

    createCable() : CableId {
        let id = this.cables.length
        let cable = new Cable(id)
        this.cables.push(cable)
        let shape = new CableShape(this.grid)
        this.cableShapes[id] = shape
        this.drawGroup.add(shape.shape)
        return id
    }

    getCable(id: CableId) : Cable {
        return this.cables[id]
    }
}