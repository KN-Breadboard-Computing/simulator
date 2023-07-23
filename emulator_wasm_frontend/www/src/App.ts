import Konva from 'konva';
import { GraphNode } from './GraphNode';
import { Cable } from './Cable';
import { Context } from './Context';

export class App {
    layer: Konva.Layer;
    cableLayer: Konva.Layer;
    cables: Cable[]

    constructor() {
        this.layer = new Konva.Layer();
        this.cableLayer = new Konva.Layer();
        this.cables = []
    }

    async run() {
        const emulator = await import('emulator');

        let graph = emulator.Graph.new()

        let c1 = graph.add_comp({ "Constant" : { "state" : false } })
        console.log(c1)

        let c2 = graph.add_comp({ "Constant" : { "state" : false } })
        console.log(c2)

        let or = graph.add_comp({"Or" : null})
        console.log(or)

        let out = graph.add_comp({ "DebugOutput" : { "state" : false } })
        console.log(out)


        graph.add_conn(c1, 0, or, 0)
        graph.add_conn(c2, 0, or, 1)

        graph.add_conn(or, 0, out, 0)

        graph.propagate(c1)
        graph.propagate(c2)

        let v = graph.get_comp(out)

        console.log(v)

        var stage = new Konva.Stage({
            container: 'container',
            width: window.innerWidth,
            height: window.innerHeight,
        });

        stage.add(this.layer);
        stage.add(this.cableLayer);

        let context: Context = new Context()
        context.addCable = this.addCable.bind(this)
        context.updateCable = this.updateCables.bind(this)
    
        var test1 = new GraphNode(0, 100,100,100,100,"Hello", 2, 1, context);
        this.layer.add(test1)
    
        var test2 = new GraphNode(1, 500,100,100,100,"Test", 2, 1, context)
        this.layer.add(test2)
    }

    addCable(a: Konva.Shape, b: Konva.Shape) {
        let cable = new Cable(a,b);
        this.cables.push(cable)
        this.cableLayer.add(cable)
    }

    updateCables() {
        for(var cable of this.cables) {
            cable.update()
        }
    }
    
}