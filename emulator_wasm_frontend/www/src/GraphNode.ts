import Konva from 'konva'
import { Cable } from './Cable'
import { Slot, SlotType } from './Slot'
import { Context } from './Context'

export let current: Slot | null = null

export class GraphNode extends Konva.Group {
    createBox(width: number, height: number) {
        var box = new Konva.Rect({
            x: 0,
            y: 0,
            width: width,
            height: height,
            fill: 'white',
            stroke: 'black',
            strokeWidth: 4,
        });
        return box;
    }

    createLabel(text: string, width: number) {
        var label = new Konva.Text({
            x: 0,
            y: 0,
            text: text,
            fontSize: 18,
            fontFamily: 'Calibri',
            fill: 'black',
            width: width,
            padding: 20,
            align: 'center',
        })
        return label;
    }

    addSlot(x: number, y: number, radius: number, color: string, slotType: SlotType, ctx: Context) {
        let slot = new Slot({
            x: x, y: y, radius: radius / 20,
            fill: color, stroke: 'black', strokeWidth: 2,
            slotType: slotType
        })
        slot.on("pointerclick", function() {
            ctx.updateSelectedSlot(this)
        })
        return slot;
    }

    constructor(id: number, x: number, y: number, width: number, height: number, text: string, input_size: number, output_size: number, ctx: Context) {
        super({ draggable: true});
        this.setPosition({x,y});

        let box = this.createBox(width, height);
        this.add(box);

        let label = this.createLabel(text, width);
        this.add(label)

        for (let i = 0; i < input_size; i++) {
            let pos_y = (i + 1) * (height / (input_size + 1))
            let pos_x = 0
            let inputSlot = this.addSlot(pos_x, pos_y, height, "red", SlotType.INPUT, ctx)
            this.add(inputSlot)
        }

        for (let i = 0; i < output_size; i++) {
            let pos_y = (i + 1) * (height / (output_size + 1))
            let pos_x = width
            let outputSlot = this.addSlot(pos_x, pos_y, height, "green", SlotType.OUTPUT, ctx)
            this.add(outputSlot)
        }

        this.on('dragmove', () => {
            ctx.updateCables()
        });
    }
}