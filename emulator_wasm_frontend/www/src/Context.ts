import { Slot } from "./Slot"

type ContextConfig = Partial<{
    addCable: Function;
    updateCables: Function;
    updateSelectedSlot: Function
}>;

export class Context {
    addCable: Function
    updateCables: Function
    updateSelectedSlot: Function

    constructor(config: ContextConfig) {
        this.addCable = config.addCable
        this.updateCables = config.updateCables
        this.updateSelectedSlot = config.updateSelectedSlot
    }
}