import { NodeId } from 'emulator'
import { Slot } from './slot'

interface ContextConfig {
    addCable?: (a: Slot, b: Slot) => void
    updateCables?: () => void
    updateSelectedSlot?: (slot: Slot) => void
    fetchFn?: (nodeId: NodeId) => any
    updateFn?: (nodeId: NodeId, state: { type: string }) => void
}

export class Context {
    addCable: (a: Slot, b: Slot) => void
    updateCables: () => void
    updateSelectedSlot: (slot: Slot) => void
    fetchFn: (nodeId: NodeId) => any
    updateFn: (nodeId: NodeId, state: { type: string }) => void

    constructor(config: ContextConfig = {}) {
        this.addCable = config.addCable || (() => {})
        this.updateCables = config.updateCables || (() => {})
        this.updateSelectedSlot = config.updateSelectedSlot || (() => {})
        this.fetchFn = config.fetchFn || (() => {})
        this.updateFn = config.updateFn || (() => {})
    }
}
