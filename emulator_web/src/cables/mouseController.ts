import { Controller, ControllerControl } from '../controllerRegister'
import { Grid } from '../grid'
import { CableId } from './cable'
import { CableGraph } from './cableGraph'
import { CableShape } from './cableShape'
import Konva from 'konva'

export type CableControllerConfig = {
    graph: CableGraph
    selection?: { cable: CableShape; point?: number }
    gridPos?: [number, number]
}

export class CableController implements Controller {
    graph: CableGraph
    selectedCable: CableShape
    selectedPoint: number
    newCable: boolean = false
    oldCablePos: Array<number> | undefined = undefined

    control: ControllerControl

    init(config: CableControllerConfig) {
        this.graph = config.graph
        if (config.selection != undefined) {
            // Sprecyzowano początkowe zaznaczenie 
            this.selectedCable = config.selection.cable
            this.oldCablePos = config.selection.cable.controlPoints.slice()
            if (config.selection.point != undefined) {
                // Sprecyzowano początkowy punkt 
                this.selectedPoint = config.selection.point
            }
        } else {
            // Nie ma początkowego zaznaczenia - trzeba utworzyć nowy kabel
            this.selectedCable = new CableShape(this.graph.grid)
            this.graph.grid.cableLayer.add(this.selectedCable.shape)
            this.newCable = true
        }

        if (this.selectedPoint == undefined && config.gridPos != undefined) {
            // Nie ma wybranego punktu, ale jest pozycja
            if(this.newCable) {
                this.selectedCable.addPoint(config.gridPos)
                this.selectedCable.addPoint(config.gridPos)
                this.selectedPoint = 1
            } else {
                let point = this.selectedCable.findOrMakePoint(config.gridPos)
                if (point != undefined) {
                    this.selectedPoint = point
                } else {
                    this.control.destroyController()
                }
            }
        } else {
            throw "Nie ma pozycji ani punktu początkowego"
        }
    }

    dragStart(pos: { x: number; y: number }) {

    }

    drag(pos: { x: number; y: number }) {
        let gridPos = this.graph.grid.worldToGrid(pos)

        this.selectedPoint = this.selectedCable.movePointAligned(this.selectedPoint, gridPos)
    }

    dragEnd(pos: { x: number; y: number }) {
        this.selectedCable.removeFlatPoints()
        if (this.newCable) {
            this.graph.createCableWithShape(this.selectedCable)
        } else {
            this.graph.cableShapeMoved(this.selectedCable, this.oldCablePos)
        }
        this.control.destroyController()
    }
}
