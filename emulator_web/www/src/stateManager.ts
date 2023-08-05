import { App } from './app'
import { GraphNode } from './graphNode'
import { Slot } from './slot'

export class StateManager {
    selectedSlot: Slot | null
    currentPopupComponent: GraphNode | null
    app: App

    constructor(app: App) {
        this.app = app
        this.selectedSlot = null
        this.currentPopupComponent = null
    }

    updateSelectedSlot(clickedSlot: Slot) {
        console.log('Clicked on slot')
        if (this.selectedSlot == null) {
            this.selectSlot(clickedSlot)
        } else if (this.selectedSlot != clickedSlot) {
            if (!Slot.areSlotsCompatible(clickedSlot, this.selectedSlot)) {
                this.selectSlot(clickedSlot)
                return
            }
            this.app.addCable(this.selectedSlot, clickedSlot)
            this.selectSlot(null)
        } else {
            this.selectSlot(null)
        }
    }

    selectSlot(slot: Slot | null) {
        this.selectedSlot?.deselect()
        this.selectedSlot = slot
        slot?.select()
    }
}
