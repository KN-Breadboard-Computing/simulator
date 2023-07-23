import Konva from 'konva';
import { GraphNode } from './shapes';

export class App {
    async run() {
        const emulator = await import('emulator');

        console.log("hello")

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
    
        // add canvas element
        var layer = new Konva.Layer();
        stage.add(layer);
    
        var test1 = new GraphNode(0, 100,100,100,100,"Hello", 2, 1)
        layer.add(test1.group)
    
        var test2 = new GraphNode(1, 500,100,100,100,"Test", 2, 1)
        layer.add(test2.group)
    }
}