import Konva from 'konva'
import { Graph } from 'emulator'
import { Cable } from './cables/cable'
import { Grid } from './grid'
import { CableGraph } from './cables/cableGraph'
import { CableController } from './cables/mouseController'
import { CacheMap } from './gridCache'
import { ControllerRegister } from './controllerRegister'

export class App {
    async run() {
        let stage = new Konva.Stage({
            container: 'container',
            width: window.innerWidth,
            height: window.innerHeight
        })

        let cableLayer = new Konva.Layer()
        let splitLayer = new Konva.Layer()
        stage.add(cableLayer,splitLayer)
        
        let controllers = new ControllerRegister(stage)
        let grid = new Grid(50, 50)
        grid.cableLayer = cableLayer
        grid.splitLayer = splitLayer
        grid.stage = stage
        grid.controllerRegister = controllers
        
        let graph = new CableGraph(grid)
        grid.graph = graph

        stage.on('pointerdown', () => {
            let pos = grid.pointerGridPos()
            if (pos != undefined) {
                controllers.registerCableController({graph: graph, gridPos: pos})
            }
        })

        stage.on('pointermove', () => {
            cableLayer.draw()
        })
    }
}
