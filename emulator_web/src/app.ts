import Konva from 'konva'
import { Graph } from 'emulator'
import { Cable } from './cables/cable'
import { Grid } from './grid'
import { CableGraph } from './cables/cableGraph'
import { CableMouseController } from './cables/mouseController'
import { CacheMap } from './gridCache'

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
        
        let grid = new Grid(50, 50)
        grid.cableLayer = cableLayer
        grid.splitLayer = splitLayer
        
        let graph = new CableGraph(grid)
        let controller = new CableMouseController(graph)

        stage.on('pointerdown', () => {
            let pos = stage.getPointerPosition()
            if (!pos) return
            controller.dragStart(pos)
        })

        stage.on('pointermove', () => {
            let pos = stage.getPointerPosition()
            if (!pos) return
            controller.drag(pos)
            cableLayer.draw()
        })

        stage.on('pointerup', () => {
            let pos = stage.getPointerPosition()
            if (!pos) return
            controller.dragEnd(pos)
        })
    }
}
