import Konva from 'konva'

export function make_component(x, y, width, height, text, input_size, output_size) {
    let group = new Konva.Group()

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
        x : x,
        y : y,
        text: text,
        fontSize: 18,
        fontFamily: 'Calibri',
        fill: 'black',
        width: width,
        padding: 20,
        align: 'center',
    })
    group.add(label)

    return group
}