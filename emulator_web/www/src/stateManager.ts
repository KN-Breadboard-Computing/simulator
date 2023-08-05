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
            console.log('No slot was previously selected')
            this.selectSlot(clickedSlot)
        } else if (this.selectedSlot != clickedSlot) {
            console.log('Selecting another slot')
            if (!Slot.areSlotsCompatible(clickedSlot, this.selectedSlot)) {
                this.selectSlot(clickedSlot)
                return
            }
            this.app.addCable(this.selectedSlot, clickedSlot)
            this.selectSlot(null)
        } else {
            console.log('Deselecting')
            this.selectSlot(null)
        }
    }

    selectSlot(slot: Slot | null) {
        this.selectedSlot?.deselect()
        this.selectedSlot = slot
        slot?.select()
    }
}
