import Konva from 'konva'
import { GridCache } from './gridCache'
import { ControllerRegister } from './controllerRegister'
import { SplitShape } from './cables/splitShape'
import { CableId } from './cables/cable'
import { CableGraph } from './cables/cableGraph'

export class Grid {
    spacingX: number
    spacingY: number
    cache: GridCache
    cableLayer: Konva.Layer
    splitLayer: Konva.Layer
    stage: Konva.Stage
    graph: CableGraph
    controllerRegister: ControllerRegister

    constructor(spacingX: number, spacingY: number) {
        this.spacingX = spacingX
        this.spacingY = spacingY
        this.cache = new GridCache()
    }

    worldToGrid(pos: { x: number; y: number }): [number, number] {
        return [Math.round(pos.x / this.spacingX), Math.round(pos.y / this.spacingY)]
    }

    gridToWorld(gridPos: [number,number]) : {x: number, y: number} {
        return {x : gridPos[0] * this.spacingX, y: gridPos[1] * this.spacingY}
    }

    pointerGridPos() : [number, number] | undefined {
        let pos = this.stage.getPointerPosition()
        if (pos != null) {
            return this.worldToGrid(pos)
        }
    }

    addSplit(gridPos: [number, number]) {
        let split = this.cache.get(gridPos).split
        
        if (split == undefined) {
            split = new SplitShape(this.gridToWorld(gridPos))
            this.cache.get(gridPos).split = split
            this.splitLayer.add(split.shape)
        } 

        split.shape.on('pointerdown', ((evt) => {
            let gridPos = this.pointerGridPos()
            if (gridPos != undefined) {
                this.removeSplit(gridPos)
                let cables = this.cablesAt(gridPos)
                for(let cable of cables) {
                    this.controllerRegister.registerCableController({
                        graph: this.graph,
                        selection: {cable: this.graph.cableShapes[cable]},
                        gridPos: gridPos
                    })
                }
            }
            evt.cancelBubble = true
        }).bind(this))
    }

    hasSplit(gridPos: [number, number]) : boolean {
        return this.cache.get(gridPos).split != undefined
    }

    removeSplit(gridPos: [number, number]) {
        let split = this.cache.get(gridPos).split
        if (split != undefined) {
            split.shape.destroy()
        }
        this.cache.get(gridPos).split = undefined
    }

    cablesAmountAt(gridPos: [number, number]) : number {
        return this.cache.get(gridPos).cables.length
    }

    cablesAt(gridPos: [number, number]) : Array<CableId> {
        return this.cache.get(gridPos).cables
    }
}
