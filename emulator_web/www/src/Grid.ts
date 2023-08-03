// Grid.ts
import Konva from 'konva';

export class Grid {
    layer: Konva.Layer;
    spacing: number;

    constructor(layer: Konva.Layer, spacing: number) {
        this.layer = layer;
        this.spacing = spacing;
        this.createGrid()
    }

    createGrid(): void {
        const width = this.layer.width();
        const height = this.layer.height();

        for (let i = 0; i < width; i += this.spacing) {
            this.layer.add(new Konva.Line({
                points: [Math.round(i), 0, Math.round(i), height],
                stroke: 'black',
                strokeWidth: 0.5,
            }));
        }

        for (let j = 0; j < height; j += this.spacing) {
            this.layer.add(new Konva.Line({
                points: [0, Math.round(j), width, Math.round(j)],
                stroke: 'black',
                strokeWidth: 0.5,
            }));
        }
    }

    getSnapToGridFunc(transform: (num: number) => number = Math.round):
    (pos: Konva.Vector2d) => Konva.Vector2d {
        return (pos: Konva.Vector2d) => ({
            x: transform(pos.x / this.spacing) * this.spacing,
            y: transform(pos.y / this.spacing) * this.spacing,
        });
    }
}
