import Konva from 'konva'

export var current = null

export class GraphNode {
    group
    #box
    #label

    constructor(id, x, y, width, height, text, input_size, output_size) {
        var group = new Konva.Group({ draggable: true })

        var box = new Konva.Rect({
            x: x,
            y: y,
            width: width,
            height: height,
            fill: 'white',
            stroke: 'black',
            strokeWidth: 4,
        });
        group.add(box)

        var label = new Konva.Text({
            x: x,
            y: y,
            text: text,
            fontSize: 18,
            fontFamily: 'Calibri',
            fill: 'black',
            width: width,
            padding: 20,
            align: 'center',
        })
        group.add(label)

        for (let i = 0; i < input_size; i++) {
            let pos_y = y + (i + 1) * (height / (input_size + 1))
            let pos_x = x
            let circle = new Konva.Circle({
                x: pos_x, y: pos_y, radius: height / 20,
                fill: "red", stroke: 'black', strokeWidth: 2,
            })
            circle.on("pointerclick", function () {
                if (current == null) {
                    current = {
                        node: id,
                        slot: i,
                        x: pos_x,
                        y: pos_y
                    }
                    console.log(current)
                } else {
                    let line = new Konva.Line({
                        points: [current.x, current.y, pos_x, pos_y],
                        stroke: "blue",
                        strokeWidth: 5
                    })
                    group.add(line)
                    current = null
                }
            })
            group.add(circle)
        }

        for (let i = 0; i < output_size; i++) {
            let pos_y = y + (i + 1) * (height / (output_size + 1))
            let pos_x = x + width
            let circle = new Konva.Circle({
                x: pos_x, y: pos_y, radius: height / 20,
                fill: "green", stroke: 'black', strokeWidth: 2,
            })
            circle.on("pointerclick", function () {
                if (current == null) {
                    current = {
                        node: id,
                        slot: i,
                        x: pos_x,
                        y: pos_y
                    }
                    console.log(current)
                } else {
                    let line = new Konva.Line({
                        points: [current.x, current.y, pos_x, pos_y],
                        stroke: "blue",
                        strokeWidth: 5
                    })
                    group.add(line)
                    current = null
                }
            })
            group.add(circle)
        }

        this.#box = box
        this.#label = label
        this.group = group
    }
}
