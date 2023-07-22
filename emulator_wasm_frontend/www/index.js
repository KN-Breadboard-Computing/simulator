//import { Graph } from "emulator"

// let graph = Graph.new()

// let c1 = graph.add_comp("const", false)

// let c2 = graph.add_comp("const", false)

// let or = graph.add_comp("or", null)

// let out = graph.add_comp("out", false)

// console.log(c1)
// console.log(c2)
// console.log(or)
// console.log(out)

// graph.add_conn(c1, 0, or, 0)
// graph.add_conn(c2, 0, or, 1)

// graph.add_conn(or, 0, out, 0)

// graph.propagate(c1)
// graph.propagate(c2)

// let v = graph.get_comp("out", out)

// console.log(v)

import Konva from 'konva'

var stage = new Konva.Stage({
    container: 'container',
    width: window.innerWidth,
    height: window.innerHeight,
  });

  // add canvas element
  var layer = new Konva.Layer();
  stage.add(layer);

  // create shape
  var box = new Konva.Rect({
    x: 50,
    y: 50,
    width: 100,
    height: 50,
    fill: '#00D2FF',
    stroke: 'black',
    strokeWidth: 4,
    draggable: true,
  });
  layer.add(box);

  // add cursor styling
  box.on('mouseover', function () {
    document.body.style.cursor = 'pointer';
  });
  box.on('mouseout', function () {
    document.body.style.cursor = 'default';
  });