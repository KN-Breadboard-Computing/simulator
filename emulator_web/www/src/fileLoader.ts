import { StageContent } from "./stageContent";

export class FileManager {
    callback: (stageContent: StageContent) => void

    constructor(callback: (stageContent: StageContent) => void) {
        this.callback = callback

        const fileInput = document.getElementById('fileInput') as HTMLInputElement;
        fileInput.addEventListener('change', async (event) => {
            const selectedFile = fileInput.files?.[0];
            
            if (selectedFile) {
                const fileContent = await this.readFileAsync(selectedFile);
                try {
                    const parsedContent = JSON.parse(fileContent);
                    if (parsedContent.nodes && parsedContent.cables) {
                        const stageContent = new StageContent();
                        stageContent.nodes = parsedContent.nodes;
                        stageContent.cables = parsedContent.cables;
                        
                        callback(stageContent);
                    } else {
                        console.error('Invalid file format.');
                    }
                } catch (error) {
                    console.error('Error parsing file content:', error);
                }
            }
        });
    }

    private readFileAsync(file: File): Promise<string> {
        return new Promise<string>((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = (event) => {
                const content = event.target?.result as string;
                resolve(content);
            };
            reader.onerror = () => {
                reject(new Error('Error reading file.'));
            };
            reader.readAsText(file);
        });
    }
        
}