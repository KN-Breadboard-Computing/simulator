import { Graph } from "emulator"
console.log("hello")

let graph = Graph.new()

let c1 = graph.add_comp({ "Constant" : { "state" : false } })
console.log(c1)

let c2 = graph.add_comp({ "Constant" : { "state" : false } })
console.log(c2)

let or = graph.add_comp({"Or" : null})
console.log(or)

let out = graph.add_comp({ "DebugOutput" : { "state" : false } })
console.log(out)


graph.add_conn(c1, 0, or, 0)
graph.add_conn(c2, 0, or, 1)

graph.add_conn(or, 0, out, 0)

graph.propagate(c1)
graph.propagate(c2)

let v = graph.get_comp(out)

console.log(v)

// import Konva from 'konva'
// import * as shapes from './shapes.js'

// var stage = new Konva.Stage({
//     container: 'container',
//     width: window.innerWidth,
//     height: window.innerHeight,
// });

// // add canvas element
// var layer = new Konva.Layer();
// stage.add(layer);

// var box = shapes.make_component(20,20,100,200,"test",10,10)
// layer.add(box)
