import Konva from "konva"
import { StateManager } from "./stateManager"

export interface PopupMenuConfig {
    stateManager: StateManager
    stage: Konva.Stage
}

export class PopupMenu {
    stateManager: StateManager
    stage: Konva.Stage

    constructor(config: PopupMenuConfig) {
        this.stateManager = config.stateManager
        this.stage = config.stage
    }

    setup() {
        let currentPopupComponent = this.stateManager.currentPopupComponent
        var menuNode = document.getElementById('menu')!
        document.getElementById('rotate-button')?.addEventListener('click', () => {
            currentPopupComponent?.group.rotate(90)
        })

        document.getElementById('delete-button')?.addEventListener('click', () => {
            currentPopupComponent?.group.destroy()
        })

        window.addEventListener('click', () => {
            menuNode.style.display = 'none'
        })

        let self = this
        this.stage.on('contextmenu', function (e) {
            e.evt.preventDefault()
            if (e.target === self.stage) {
                return
            }
            currentPopupComponent = e.target.getParent()

            menuNode.style.display = 'initial'
            var containerRect = self.stage.container().getBoundingClientRect()
            menuNode.style.top = containerRect.top + self.stage.getPointerPosition()!.y + 4 + 'px'
            menuNode.style.left = containerRect.left + self.stage.getPointerPosition()!.x + 4 + 'px'
        })
    }
}