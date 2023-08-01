import konva from "konva"
import { Text } from "konva/lib/shapes/Text"

type updateFn = (this: ComponentInfo, state_fetch: () => any, state_update: (any) => void, draw_state: { group: konva.Group }) => void

export type ComponentInfo = {
    type: string,
    width: number,
    height: number,
    inputSize: number,
    outputSize: number,
    on_start: updateFn,
    on_update: updateFn,
}

const ComponentDefaults: ComponentInfo = {
    type: undefined,
    inputSize: undefined,
    outputSize: undefined,
    width: 150,
    height: 70,
    on_start: function (this, state_fetch, state_update, draw_state): void {
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
        draw_state.group.add(label)
    },
    on_update: function (): void { },
}

function constantStart(this: ComponentInfo, state_fetch: () => { state: boolean }, state_update: (a: { state: boolean }) => void, draw_state: { group: konva.Group }): void {
    let state = state_fetch().state

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
    draw_state.group.add(label)

    draw_state.group.on('pointerclick', function () {
        let state = state_fetch().state
        state_update({state: !state})
    })
}

function debugStart(this: ComponentInfo, state_fetch: () => { state: boolean }, state_update: (a: { state: boolean }) => void, draw_state: { group: konva.Group }): void {
    let state = state_fetch().state

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
    draw_state.group.add(label)
}

function constantUpdate(this: ComponentInfo, state_fetch: () => { state: boolean }, state_update: (a: { state: boolean }) => void, draw_state: { group: konva.Group }): void {
    let state = state_fetch().state
    let label = draw_state.group.find<Text>("Text")
    label[0].text(state ? '1' : '0')
}

function debugUpdate(this: ComponentInfo, state_fetch: () => { state: boolean }, state_update: (a: { state: boolean }) => void, draw_state: { group: konva.Group }): void {
    let state = state_fetch().state
    let label = draw_state.group.find<Text>("Text")
    label[0].text(state ? '1' : '0')
}

export const components: Array<ComponentInfo> = [
    { ...ComponentDefaults, type: "And", inputSize: 2, outputSize: 1, },
    { ...ComponentDefaults, type: "Or", inputSize: 2, outputSize: 1 },
    { ...ComponentDefaults, type: "Not", inputSize: 2, outputSize: 1 },
    { ...ComponentDefaults, type: "Xor", inputSize: 2, outputSize: 1 },
    { ...ComponentDefaults, type: "Constant", inputSize: 0, outputSize: 1, width: 50, height: 50, on_start: constantStart, on_update: constantUpdate },
    { ...ComponentDefaults, type: "DebugOutput", inputSize: 1, outputSize: 0, width: 50, height: 50, on_start: debugStart, on_update: debugUpdate },
    { ...ComponentDefaults, type: "Fork", inputSize: 1, outputSize: 2, width: 50, height: 50 },
]