import konva from 'konva'
import { GraphNodeRectangleShape, GraphNodeShape, GraphNodeTriangleShape } from './graphNodeShape'
import { GraphNodeBuilder } from './graphNodeBuilder'
import { GraphNode } from './graphNode'
import { Text } from 'konva/lib/shapes/Text'

export interface Tag {
    addToBuild(builder: GraphNodeBuilder): void
}

class LabelTag implements Tag {
    label: string
    constructor(label: string) {
        this.label = label
    }
    addToBuild(builder: GraphNodeBuilder): void {
        builder.setLabel(this.label)
    }
}

class BitUpdateTag implements Tag {
    addToBuild(builder: GraphNodeBuilder): void {
        builder.setLabel('0')

        builder.setOnNodeUpdate<{ state: boolean }>(function (f, u): void {
            let label: Text = this.shapeGroup.findOne('#mainLabel')
            label.text(f().state ? '1' : '0')
        })
    }
}

class BitFlipClickTag implements Tag {
    addToBuild(builder: GraphNodeBuilder): void {
        builder.setOnClick<{ state: boolean }>((f, u) => {
            let state = f().state
            u({ state: !state })
        })
    }
}

export type NodeEventListener<T> = (this: GraphNode, fetchFn: () => T, updateFn: (state: T) => void) => void

export type ComponentMeta = {
    type: string
    inputSize: number
    outputSize: number
    shape: GraphNodeShape
    tags: Tag[]
}

export const components: ComponentMeta[] = [
    { type: 'And', inputSize: 2, outputSize: 1, shape: new GraphNodeRectangleShape(4, 4), tags: [new LabelTag('And')] },
    { type: 'Or', inputSize: 2, outputSize: 1, shape: new GraphNodeRectangleShape(4, 4), tags: [new LabelTag('Or')] },
    { type: 'Xor', inputSize: 2, outputSize: 1, shape: new GraphNodeRectangleShape(4, 4), tags: [new LabelTag('Xor')] },
    { type: 'Not', inputSize: 1, outputSize: 1, shape: new GraphNodeTriangleShape(2, 2), tags: [] },
    {
        type: 'Constant',
        inputSize: 0,
        outputSize: 1,
        shape: new GraphNodeRectangleShape(2, 2),
        tags: [new BitFlipClickTag(), new BitUpdateTag()]
    },
    {
        type: 'DebugOutput',
        inputSize: 1,
        outputSize: 0,
        shape: new GraphNodeRectangleShape(2, 2),
        tags: [new BitUpdateTag()]
    },
    { type: 'Fork', inputSize: 1, outputSize: 2, shape: new GraphNodeRectangleShape(2, 4), tags: [] }
]
