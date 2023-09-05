import Konva from 'konva'
import { Graph } from 'emulator'
import { Cable } from './cables/cable'
import { Grid } from './grid'
import { CableGraph } from './cables/cableGraph'
import { CableMouseController } from './cables/mouseController'

export class App {
    async run() {
        let stage = new Konva.Stage({
            container: 'container',
            width: window.innerWidth,
            height: window.innerHeight
        })

        let layer = new Konva.Layer()
        stage.add(layer)

        let grid = new Grid(50, 50)

        let graph = new CableGraph(grid)
        let controller = new CableMouseController(graph,grid,layer)

        stage.on('pointerdown', () => {
            console.log("aaaa")
            let pos = stage.getPointerPosition()
            if (!pos) return
            controller.dragStart(pos)
        })

        stage.on('pointermove', () => {
            let pos = stage.getPointerPosition()
            if (!pos) return
            controller.drag(pos)
            layer.draw()
        })

        stage.on('pointerup', () => {
            let pos = stage.getPointerPosition()
            if (!pos) return
            console.log(controller.selectedCable)
            controller.dragEnd(pos)
        })
    }
}
