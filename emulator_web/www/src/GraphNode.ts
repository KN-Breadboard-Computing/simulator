import Konva from 'konva';
import { InputSlot, OutputSlot, Slot, SlotType } from './Slot';
import { Context } from './Context';
import { NodeId } from 'emulator';
import { ComponentInfo } from './component_list';

export interface GraphNodeConfig {
    node_id: NodeId,
    x: number,
    y: number,
    componentInfo: ComponentInfo,
    context: Context,
    onHover?: () => void,
    offHover?: () => void,
    onClick?: () => void,
    snapToGrid?: (pos: Konva.Vector2d) => Konva.Vector2d
}

export class GraphNode extends Konva.Group {
    private context: Context;
    private onHover: () => void;
    private offHover: () => void;
    private onClick: () => void;

    nodeId: NodeId;
    componentInfo: ComponentInfo;
    inputSlots: InputSlot[] = [];
    outputSlots: OutputSlot[] = [];

    constructor(config: GraphNodeConfig) {
        super({ draggable: true, dragBoundFunc: config.snapToGrid , offset: {x : config.componentInfo.width/2, y: config.componentInfo.height/2}});

        this.nodeId = config.node_id
        this.componentInfo = config.componentInfo

        this.setPosition({ x: config.x, y: config.y });

        this.width(config.componentInfo.width);
        this.height(config.componentInfo.height);
        this.context = config.context;
        this.onHover = config.onHover || (() => { });
        this.offHover = config.offHover || (() => { });
        this.onClick = config.onClick || (() => { });

        this.add(this.createBox());
        //this.add(this.createLabel(config.text));
        this.componentInfo.on_start(() => { return this.context.fetchFn(this.nodeId) }, (a) => { this.context.updateFn(this.nodeId, { type: this.componentInfo.type, ...a }) }, { group: this })

        this.addSlots(config.componentInfo.inputSize, config.componentInfo.outputSize);

        this.on('dragmove', () => this.context.updateCables());
        this.on('mouseover', this.onHover);
        this.on('mouseout', this.offHover);
        this.on('click', this.onClick);
    }

    updateNodeState() {
        console.log("aaa")
        this.componentInfo.on_update(() => { return this.context.fetchFn(this.nodeId) }, (a) => { this.context.updateFn(this.nodeId, { type: this.componentInfo.type, ...a }) }, { group: this })
    }

    private createBox(): Konva.Rect {
        return new Konva.Rect({
            x: 0,
            y: 0,
            width: this.width(),
            height: this.height(),
            fill: 'white',
            stroke: 'black',
            strokeWidth: 4
        });
    }

    private createLabel(text: string): Konva.Text {
        return new Konva.Text({
            x: 0,
            y: 0,
            text: text,
            fontSize: 18,
            fontFamily: 'Calibri',
            fill: 'black',
            width: this.width(),
            padding: 20,
            align: 'center',
        });
    }

    private addSlots(inputSize: number, outputSize: number): void {
        for (let i = 0; i < inputSize; i++) {
            let slot = this.createSlot(i, inputSize, 0, "red", SlotType.INPUT);
            this.add(slot);
            this.inputSlots.push(slot as InputSlot);
        }

        for (let i = 0; i < outputSize; i++) {
            let slot = this.createSlot(i, outputSize, this.width(), "green", SlotType.OUTPUT);
            this.add(slot);
            this.outputSlots.push(slot as OutputSlot);
        }
    }

    private createSlot(i: number, total: number, x: number, color: string, type: SlotType): InputSlot | OutputSlot {
        const y = (i + 1) * (this.height() / (total + 1));
        const config = {
            x: x,
            y: y,
            radius: 5,
            fill: color,
            stroke: 'black',
            strokeWidth: 2
        }
        let slot: InputSlot | OutputSlot;

        if (type === SlotType.OUTPUT) {
            slot = new OutputSlot(config, this.nodeId, i);
        } else {
            slot = new InputSlot(config, this.nodeId, i);
        }

        slot.on('click', () => this.context.updateSelectedSlot(slot));
        return slot;
    }
}
