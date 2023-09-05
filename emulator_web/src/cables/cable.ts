export type CableId = number

export class Cable {
    id: CableId
    neighbours: [Array<CableId>, Array<CableId>]
    connectedComponents: Array<number>

    constructor(id: number) {
        this.id = id
        this.neighbours = [[],[]]
        this.connectedComponents = []
    }
}
