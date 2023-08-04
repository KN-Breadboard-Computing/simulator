import konva from 'konva'
import { Text } from 'konva/lib/shapes/Text'

type updateFn = (
    this: ComponentInfo,
    stateFetch: () => any,
    stateUpdate: (any) => void,
    drawState: { group: konva.Group }
) => void

export type ComponentInfo = {
    type: string
    width: number
    height: number
    inputSize: number
    outputSize: number
    onStart: updateFn
    onUpdate: updateFn
}

const ComponentDefaults: ComponentInfo = {
    type: undefined,
    inputSize: undefined,
    outputSize: undefined,
    width: 4,
    height: 2,
    onStart: function (this, stateFetch, stateUpdate, drawState): void {
        var label = new konva.Text({
            text: this.type,
            fontSize: 18,
            fontFamily: 'Arial',
            fill: 'black',
            width: this.width,
            height: this.height,
            padding: 20,
            align: 'center',
            verticalAlign: 'middle'
        })
        drawState.group.add(label)
    },
    onUpdate: function (): void {}
}

function constantStart(
    this: ComponentInfo,
    stateFetch: () => { state: boolean },
    stateUpdate: (a: { state: boolean }) => void,
    drawState: { group: konva.Group }
): void {
    let state = stateFetch().state

    var label = new konva.Text({
        text: state ? '1' : '0',
        fontSize: 18,
        fontFamily: 'Arial',
        fill: 'black',
        width: this.width,
        height: this.height,
        align: 'center',
        verticalAlign: 'middle'
    })
    drawState.group.add(label)

    drawState.group.on('pointerclick', function () {
        let state = stateFetch().state
        stateUpdate({ state: !state })
    })
}

function debugStart(
    this: ComponentInfo,
    stateFetch: () => { state: boolean },
    stateUpdate: (a: { state: boolean }) => void,
    drawState: { group: konva.Group }
): void {
    let state = stateFetch().state

    var label = new konva.Text({
        text: state ? '1' : '0',
        fontSize: 18,
        fontFamily: 'Arial',
        fill: 'black',
        width: this.width,
        height: this.height,
        align: 'center',
        verticalAlign: 'middle'
    })
    drawState.group.add(label)
}

function constantUpdate(
    this: ComponentInfo,
    stateFetch: () => { state: boolean },
    stateUpdate: (a: { state: boolean }) => void,
    drawState: { group: konva.Group }
): void {
    let state = stateFetch().state
    let label = drawState.group.find<Text>('Text')
    label[0].text(state ? '1' : '0')
}

function debugUpdate(
    this: ComponentInfo,
    stateFetch: () => { state: boolean },
    stateUpdate: (a: { state: boolean }) => void,
    drawState: { group: konva.Group }
): void {
    let state = stateFetch().state
    let label = drawState.group.find<Text>('Text')
    label[0].text(state ? '1' : '0')
}

export const components: Array<ComponentInfo> = [
    { ...ComponentDefaults, type: 'And', inputSize: 2, outputSize: 1 },
    { ...ComponentDefaults, type: 'Or', inputSize: 2, outputSize: 1 },
    { ...ComponentDefaults, type: 'Not', inputSize: 2, outputSize: 1 },
    { ...ComponentDefaults, type: 'Xor', inputSize: 2, outputSize: 1 },
    {
        ...ComponentDefaults,
        type: 'Constant',
        inputSize: 0,
        outputSize: 1,
        width: 2,
        height: 2,
        onStart: constantStart,
        onUpdate: constantUpdate
    },
    {
        ...ComponentDefaults,
        type: 'DebugOutput',
        inputSize: 1,
        outputSize: 0,
        width: 2,
        height: 2,
        onStart: debugStart,
        onUpdate: debugUpdate
    },
    { ...ComponentDefaults, type: 'Fork', inputSize: 1, outputSize: 2, width: 50, height: 50 }
]
