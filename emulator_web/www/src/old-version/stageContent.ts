import { Cable } from './cable'
import { GraphNode } from './graphNode'

export interface StageContentConfig {
    nodes?: GraphNode[]
    cables?: Cable[]
}

export class StageContent {
    nodes: GraphNode[]
    cables: Cable[]

    constructor(config: StageContentConfig = {}) {
        this.nodes = config.nodes ? config.nodes : []
        this.cables = config.cables ? config.cables : []
    }
}
