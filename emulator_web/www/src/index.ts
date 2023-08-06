import { App } from './old-version/app'

const app = new App()
app.run().catch(err => console.error(err))
