import { Slot } from "./Slot"

interface ContextConfig {
    addCable?: (a: Slot, b: Slot) => void;
    updateCables?: () => void;
    updateSelectedSlot?: (slot: Slot) => void;
};

export class Context {
    addCable: (a: Slot, b: Slot) => void;
    updateCables: () => void;
    updateSelectedSlot: (slot: Slot) => void;

    constructor(config: ContextConfig = {}) {
        this.addCable = config.addCable || (() => {});
        this.updateCables = config.updateCables || (() => {});
        this.updateSelectedSlot = config.updateSelectedSlot || (() => {});
    }
}