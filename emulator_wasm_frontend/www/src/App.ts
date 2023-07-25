import Konva from 'konva';
import { GraphNode, GraphNodeConfig } from './GraphNode';
import { Cable } from './Cable';
import { Context } from './Context';
import { Slot } from './Slot';
import { components } from './component_list';
import { selected, setup_side_panel } from './side_panel';
import { Vector2d } from 'konva/lib/types';
import { Graph, NodeId } from 'emulator';

export class App {
    componentLayer: Konva.Layer;
    cableLayer: Konva.Layer;

    graph: Graph
    nodes: GraphNode[]
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

        this.graph = emulator.Graph.new()

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
    

        let app = this
        stage.on("pointerclick", function () {
            let pos = stage.getPointerPosition()

            if (selected != null) {
                app.addNode({
                    node_id: app.graph.add_comp({"type" : selected.component.type}),
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
            cable.update()
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

    propagate(node_id: NodeId) {
        this.graph.propagate(node_id)
        this.updateAllNodes()
    }

    updateAllNodes() {
        for (const node of this.nodes) {
            let output_state = this.graph.output_state(node.node_id)

            for (const output of node.outputSlots) {
                let bit = output_state & 1
                output_state = output_state >> 1
                output.setValue(bit == 1)
            }
        }
    }
}