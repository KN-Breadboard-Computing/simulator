import { ComponentInfo, components } from './componentList'

export var selected: { button: HTMLElement; component: ComponentInfo } = null

export function unselect() {
    if (selected != null) {
        selected.button.classList.remove('selected')
    }
    selected = null
}

export function select(toSelect: { button: HTMLElement; component: ComponentInfo }) {
    if (selected != null) {
        if (selected.button === toSelect.button) {
            unselect()
            return
        }
        selected.button.classList.remove('selected')
    }
    toSelect.button.classList.add('selected')
    selected = toSelect
}

export function setupSidePanel() {
    const buttonPanel = document.getElementById('button_panel')

    for (const c of components) {
        const button = document.createElement('button')
        button.className = 'comp_button'
        button.textContent = c.type
        button.onclick = function () {
            console.log('Clicked on', c.type)
            select({ button: button, component: c })
        }

        buttonPanel.appendChild(button)
    }
}
