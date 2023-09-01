import Konva from 'konva'
import { Graph } from 'emulator'
import { Cable } from './cables/cable'
import { CableDrawGroup } from './cables/cableDrawGroup'
import { Grid } from './grid'
import { CableGraph } from './cables/cableGraph'


export class App {
    async run() {
        let stage = new Konva.Stage({
            container: 'container',
            width: window.innerWidth,
            height: window.innerHeight
        })

        let layer = new Konva.Layer()
        stage.add(layer)

        let grid = new Grid(50,50)
        let graph = new CableGraph(grid)
        layer.add(graph.drawGroup.group)

        let selected : [number,number] | null = null

        
        stage.on('pointerdown', () => {
            let pos = stage.getPointerPosition()
            if (!pos) return
            let gridPos = grid.worldToGrid(pos)
            selected = [graph.createCable(),1]
            graph.updateCable(selected[0], (cable) => {
                cable.addPoint(0,gridPos)
                cable.addPoint(1,gridPos)
                console.log(cable)
            })
            let id = selected[0]
            graph.drawGroup.onCableLine(graph.cables[id], 'pointerdown', (evt) => {
                let cable = graph.cables[id]
                let pos = stage.getPointerPosition()
                if (!pos) return
                let gridPos = grid.worldToGrid(pos)
                let newp = cable.pointToSegment(gridPos)!!
                cable.addPoint(newp+1,gridPos)
                selected = [id,newp+1]
                evt.cancelBubble = true
            })
        })

        stage.on('pointermove', () => {
            if (selected) {
                let pos = stage.getPointerPosition()
                if (!pos) return
                let gridPos = grid.worldToGrid(pos)
                console.log(gridPos)
                graph.updateCable(selected[0], (cable) => {
                    selected!![1] = cable.movePointAligned(selected!![1], gridPos)
                    console.log(selected)
                })
            }
        })

        stage.on('pointerup', () => {
            console.log(graph.cables[selected!![0]])
            selected = null
        })
    }
}