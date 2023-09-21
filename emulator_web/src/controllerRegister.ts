import Konva from 'konva'
import { CableController, CableControllerConfig } from './cables/mouseController'

type Vec = { x: number; y: number }

export type ControllerControl = {
    destroyController(): void
}

export interface Controller {
    control: ControllerControl
    init(config): void 
    dragStart(v: Vec): void
    drag(v: Vec): void
    dragEnd(v: Vec): void
}

let controllerTypes = { cable: CableController }
type controllerTypesNames = keyof typeof controllerTypes

export class ControllerRegister {
    controllers: Array<Controller>

    constructor(stage: Konva.Stage) {
        stage.on(
            'pointerdown',
            (() => {
                let pos = stage.getPointerPosition()
                if (pos != null) {
                    for (let c of this.controllers) {
                        if (c != undefined) {
                            c.dragStart(pos)
                        }
                    }
                }
            }).bind(this)
        )

        stage.on(
            'pointermove',
            (() => {
                let pos = stage.getPointerPosition()
                if (pos != null) {
                    for (let c of this.controllers) {
                        if (c != undefined) {
                            c.drag(pos)
                        }
                    }
                }
            }).bind(this)
        )

        stage.on(
            'pointerup',
            (() => {
                let pos = stage.getPointerPosition()
                if (pos != null) {
                    for (let c of this.controllers) {
                        if (c != undefined) {
                            c.dragEnd(pos)
                        }
                    }
                }
            }).bind(this)
        )

        this.controllers = []
    }

    registerController(type: controllerTypesNames, config) {
        let controller = new controllerTypes[type]()
        controller.control = {
            destroyController: (() => {
                this.removeController(id)
            }).bind(this)
        }
        controller.init(config)

        let id = this.controllers.length
        this.controllers.push(controller)
        
    }

    //Helpers 
    //Boilerplate, który można usunąć, gdy się ogarnie lepiej typescripta
    registerCableController(config: CableControllerConfig) {
        this.registerController('cable', config)
    }

    removeController(id: number) {
        delete this.controllers[id]
    }
}
