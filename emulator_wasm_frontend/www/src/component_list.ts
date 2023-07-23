export interface Component {
    type : string,
    input_size: number,
    output_size: number,
}

export var components : Array<Component> = [
    {type : "And", input_size: 2, output_size: 1},
    {type : "Or", input_size: 2, output_size: 1},
    {type : "Not", input_size: 2, output_size: 1},
    {type : "Xor", input_size: 2, output_size: 1},
    {type : "Constant", input_size: 0, output_size: 1},
    {type : "DebugOutput", input_size: 1, output_size: 0},
    {type : "Fork", input_size: 1, output_size: 2},
]