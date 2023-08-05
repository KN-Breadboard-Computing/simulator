import { Cable } from "./cable"
import { GraphNode } from "./graphNode"

export class StageContent {
    nodes: GraphNode[]
    cables: Cable[]

    constructor() {
        this.cables = []
        this.nodes = []
    }
}