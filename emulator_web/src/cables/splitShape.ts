import Konva from "konva";

export class SplitShape {
    shape: Konva.Shape

    constructor(position: {x: number, y: number}) {
        this.shape = new Konva.Circle({
            x: position.x,
            y: position.y,
            radius: 8,
            stroke: 'black',
            fill: 'gray',
            strokeWidth: 5
        })
    }
}