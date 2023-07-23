import Konva from 'konva';
import { GraphNode } from './shapes';
import { components } from './component_list';
import { selected, setup_side_panel } from './side_panel';

export class App {
    async run() {
        setup_side_panel()

        const emulator = await import('emulator');

        console.log("hello")

        let graph = emulator.Graph.new()

        let c1 = graph.add_comp({ "type": "Constant"})
        console.log(c1)

        let c2 = graph.add_comp({ "type": "Constant", "state" : true})
        console.log(c2)

        let or = graph.add_comp({ "type": "Or" })
        console.log(or)

        let out = graph.add_comp({ "type": "DebugOutput"})
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

        stage.on("pointerclick", function () {
            let pos = stage.getPointerPosition()

            if (selected != null) {
                var comp = new GraphNode(0, pos.x - selected.component.width/2, pos.y - selected.component.height/2, selected.component.width, selected.component.height, selected.component.type, selected.component.input_size, selected.component.output_size)
                layer.add(comp.group)
            }
            
        })
    }
}