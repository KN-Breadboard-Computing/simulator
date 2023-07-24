import Konva from 'konva';
import { GraphNode } from './GraphNode';
import { Cable } from './Cable';
import { Context } from './Context';
import { Slot } from './Slot';
import { components } from './component_list';
import { selected, setup_side_panel } from './side_panel';

export class App {
    componentLayer: Konva.Layer;
    cableLayer: Konva.Layer;
    cables: Cable[]
    selectedSlot: Slot | null

    constructor() {
        this.componentLayer = new Konva.Layer();
        this.cableLayer = new Konva.Layer();
        this.cables = []
        this.selectedSlot = null
    }

    async run() {
        setup_side_panel()

        const emulator = await import('emulator');

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

        stage.add(this.componentLayer);
        stage.add(this.cableLayer);

        let context: Context = new Context({
            addCable: this.addCable.bind(this),
            updateCables: this.updateCables.bind(this),
            updateSelectedSlot: this.updateSelectedSlot.bind(this)
        })
    
        var test1 = new GraphNode(0, 100,100,100,100,"Hello", 2, 1, context);
        this.componentLayer.add(test1)
    
        var test2 = new GraphNode(1, 500,100,100,100,"Test", 2, 1, context)
        this.componentLayer.add(test2)
        this.componentLayer.add(test2)

        let app = this
        stage.on("pointerclick", function () {
            let pos = stage.getPointerPosition()

            if (selected != null) {
                var comp = new GraphNode(0, pos.x - selected.component.width/2, pos.y - selected.component.height/2, selected.component.width, selected.component.height, selected.component.type, selected.component.input_size, selected.component.output_size, context)
                app.componentLayer.add(comp)
            }
        })
    }

    addCable(a: Slot, b: Slot) {
        if(a.slotType == b.slotType) {
            console.error("Can't connect slots of the same type");
            return;
        }
        let cable = new Cable(a,b);
        this.cables.push(cable)
        this.cableLayer.add(cable)
    }

    updateCables() {
        for(var cable of this.cables) {
            cable.update()
        }
    }

    updateSelectedSlot(clickedSlot: Slot) {
        if (this.selectedSlot == null) {
            this.selectSlot(clickedSlot)
        } else if (this.selectedSlot != clickedSlot) {
            this.addCable(this.selectedSlot, clickedSlot)
            this.selectSlot(null)
        } else {
            this.selectSlot(null)
        }
    }

    selectSlot(slot: Slot) {
        this.selectedSlot?.deselect();
        this.selectedSlot = slot;
        slot?.select();
    }
}