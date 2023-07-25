import Konva from 'konva';
import { GraphNode, GraphNodeConfig } from './GraphNode';
import { Cable } from './Cable';
import { Context } from './Context';
import { Slot } from './Slot';
import { components } from './component_list';
import { selected, setup_side_panel } from './side_panel';
import { Vector2d } from 'konva/lib/types';

export class App {
    componentLayer: Konva.Layer;
    cableLayer: Konva.Layer;

    nodes: GraphNode[]
    cables: Cable[]
    selectedSlot: Slot | null

    constructor() {
        this.componentLayer = new Konva.Layer();
        this.cableLayer = new Konva.Layer();
        this.cables = []
        this.nodes = []
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
    
        this.addNode({
            id: 0,
            x: 100,
            y: 100,
            width: 100,
            height: 100,
            text: "Hello",
            inputSize: 2,
            outputSize: 1,
            context: context
        })

        this.addNode({
            id: 1,
            x: 500,
            y: 100,
            width: 100,
            height: 100,
            text: "Test",
            inputSize: 2,
            outputSize: 1,
            context: context
        })

        let app = this
        stage.on("pointerclick", function () {
            let pos = stage.getPointerPosition()

            if (selected != null) {
                app.addNode({
                    id: 0,
                    x: pos.x - selected.component.width / 2,
                    y: pos.y - selected.component.height / 2,
                    width: selected.component.width,
                    height: selected.component.height,
                    text: selected.component.type,
                    inputSize: selected.component.input_size,
                    outputSize: selected.component.output_size,
                    context: context
                })
            }
        })
    }

    addNode(config: GraphNodeConfig) {
        let comp = new GraphNode(config)
        this.nodes.push(comp)
        this.componentLayer.add(comp)
    }

    addCable(a: Slot, b: Slot) {
        if(!this.areSlotsCompatible(a,b)) {
            // console.error("Can't connect slots of the same type");
            return;
        }
        let cable = new Cable(a,b);
        this.cables.push(cable)
        this.cableLayer.add(cable)
    }

    areSlotsCompatible(a: Slot, b: Slot) {
        if(a.slotType == b.slotType) {
            return false;
        }
        return true;
    }

    updateCables() {
        for(var cable of this.cables) {
            cable.updatePosition()
        }
    }

    updateSelectedSlot(clickedSlot: Slot) {
        if (this.selectedSlot == null) {
            this.selectSlot(clickedSlot)
        } else if (this.selectedSlot != clickedSlot) {
            if (!this.areSlotsCompatible(clickedSlot, this.selectedSlot)) {
                this.selectSlot(clickedSlot)
                return;
            }
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