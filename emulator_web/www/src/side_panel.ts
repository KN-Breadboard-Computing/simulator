import { ComponentInfo, components } from "./component_list"

export var selected : { button: HTMLElement, component: ComponentInfo} = null

export function unselect() {
    if (selected != null) {
        selected.button.classList.remove("selected")
    }
    selected = null
}

export function select(to_select: { button: HTMLElement, component: ComponentInfo}) {
    if (selected != null) {
        if (selected.button === to_select.button) {
            unselect()
            return
        }
        selected.button.classList.remove("selected")
    }
    to_select.button.classList.add("selected")
    selected = to_select
}


export function setup_side_panel() {
    const button_panel = document.getElementById("button_panel")

    for (const c of components) {
        const button = document.createElement("button")
        button.className = "comp_button"
        button.textContent = c.type
        button.onclick = function () {
            console.log("Clicked on", c.type)
            select({button: button, component: c})
        }

        button_panel.appendChild(button)
    }
}