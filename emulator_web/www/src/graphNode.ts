import Konva from 'konva'
import { InputSlot, OutputSlot, Slot } from './slot'
import { Context } from './context'
import { NodeId } from 'emulator'

export interface GraphNodeConfig {
    nodeId: NodeId
    context: Context
    x: number
    y: number
}

export class GraphNode extends Konva.Group {
    context: Context
    nodeId: NodeId

    onNodeUpdate?: (this: GraphNode) => void

    inputSlots: InputSlot[] = []
    outputSlots: OutputSlot[] = []

    constructor(config: GraphNodeConfig) {
        super({
            draggable: true
        })

        this.context = config.context
        this.nodeId = config.nodeId
        this.setPosition({ x: config.x, y: config.y })
        this.on('dragmove', () => this.context.updateCables())
    }

    addSlot(slot: InputSlot | OutputSlot): void {
        if (slot instanceof InputSlot) {
            console.log('Adding an input slot')
            this.inputSlots.push(slot as InputSlot)
        } else {
            console.log('Adding an output slot')
            this.outputSlots.push(slot as OutputSlot)
        }
        this.add(slot)
    }

    updateNodeState() {
        if (this.onNodeUpdate != undefined) {
            this.onNodeUpdate()
        }
    }
}
