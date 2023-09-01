import { Grid } from "../grid";
import { Cable, CableId } from "./cable";
import { CableDrawGroup } from "./cableDrawGroup";

export class CableGraph {
    cables : Cable[]
    grid: Grid
    drawGroup: CableDrawGroup

    constructor(grid: Grid) {
        this.cables = []
        this.grid = grid
        this.drawGroup = new CableDrawGroup(grid)
    }

    createCable() : CableId {
        let id = this.cables.length
        let cable = new Cable(id)
        this.cables.push(cable)
        this.drawGroup.addCableLine(cable)
        return id
    }

    updateCable(id: CableId, func : (cable: Cable) => void) {
        func(this.cables[id])
        this.drawGroup.updateCableLine(this.cables[id])
    }
}