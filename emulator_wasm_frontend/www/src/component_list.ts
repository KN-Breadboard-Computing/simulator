import konva from "konva"

export interface Component {
    type: string,
    width: number,
    height: number,
    input_size: number,
    output_size: number,
    on_start: (comp_state: any, draw_state: {group: konva.Group, width: number, height: number}) => void,
    on_update: (comp_state: any, draw_state: {group: konva.Group, width: number, height: number}) => void,
}

const ComponentDefaults: Component = {
    type: "oops",
    width: 150,
    height: 70,
    input_size: 1,
    output_size: 1,
    on_start: (comp_state, draw_state) => {
        var label = new konva.Text({
            text: comp_state.type,
            fontSize: 18,
            fontFamily: 'Arial',
            fill: 'black',
            width: draw_state.width,
            height: draw_state.height,
            padding: 20,
            align: 'center',
        })
        draw_state.group.add(label)
    },
    on_update: null,
}

export const components: Array<Component> = [
    { ...ComponentDefaults, type: "And", input_size: 2, output_size: 1, },
    { ...ComponentDefaults, type: "Or", input_size: 2, output_size: 1 },
    { ...ComponentDefaults, type: "Not", input_size: 2, output_size: 1 },
    { ...ComponentDefaults, type: "Xor", input_size: 2, output_size: 1 },
    { ...ComponentDefaults, type: "Constant", input_size: 0, output_size: 1, width: 50, height: 50 },
    { ...ComponentDefaults, type: "DebugOutput", input_size: 1, output_size: 0, width: 50, height: 50 },
    { ...ComponentDefaults, type: "Fork", input_size: 1, output_size: 2, width: 50, height: 50 },
]