import { StageContent } from './stageContent'

export interface FileManagerConfig {
    loadCallback: (stageContent: StageContent) => void
    saveCallback: () => StageContent
}

export class FileManager {
    loadCallback: (stageContent: StageContent) => void
    saveCallback: () => StageContent

    constructor(config: FileManagerConfig) {
        this.loadCallback = config.loadCallback
        this.saveCallback = config.saveCallback

        this.setFileInput()
        this.setFileOutput()
    }

    private setFileOutput() {
        const fileOutput = document.getElementById('fileOutput') as HTMLButtonElement
        fileOutput.onclick = () => {
            let stageContent = this.saveCallback()
            const blob = new Blob([JSON.stringify(stageContent)], { type: 'application/json' })
            const blobUrl = URL.createObjectURL(blob)

            const link = document.createElement('a')
            link.href = blobUrl
            link.download = 'data.json'

            // Add the link to the DOM and simulate a click to trigger the download
            link.style.display = 'none'
            document.body.appendChild(link)
            link.click()

            // Clean up
            document.body.removeChild(link)
            URL.revokeObjectURL(blobUrl)
        }
    }

    private setFileInput() {
        const fileInput = document.getElementById('fileInput') as HTMLInputElement
        fileInput.addEventListener('change', async event => {
            const selectedFile = fileInput.files?.[0]

            if (selectedFile) {
                const fileContent = await this.readFileAsync(selectedFile)
                try {
                    const parsedContent = JSON.parse(fileContent)
                    const stageContent = new StageContent()
                    stageContent.nodes = parsedContent.nodes
                    stageContent.cables = parsedContent.cables

                    console.log(stageContent)
                    this.loadCallback(stageContent)
                } catch (error) {
                    console.error('Error parsing file content:', error)
                }
            }
        })
    }

    private readFileAsync(file: File): Promise<string> {
        return new Promise<string>((resolve, reject) => {
            const reader = new FileReader()
            reader.onload = event => {
                const content = event.target?.result as string
                resolve(content)
            }
            reader.onerror = () => {
                reject(new Error('Error reading file.'))
            }
            reader.readAsText(file)
        })
    }
}
