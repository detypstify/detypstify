import init, {addTwoNumbers, getPowers, logHelloWorld} from "../build_artifacts/wasm/recognize.js";
import memory from "../build_artifacts/wasm/recognize.js";

document.addEventListener("DOMContentLoaded", async function () {
    const canvas = document.getElementById("drawingCanvas") as HTMLCanvasElement;
    const ctx = canvas.getContext("2d")!;
    let isDrawing = false;

    function startDrawing(event: MouseEvent) {
        isDrawing = true;
        draw(event); }

        function draw(event: MouseEvent) {
            if (!isDrawing) return;
            ctx.lineWidth = 3;
            ctx.lineCap = "round";
            ctx.lineTo(event.offsetX, event.offsetY);
            ctx.stroke();
            ctx.beginPath();
            ctx.moveTo(event.offsetX, event.offsetY);
        }

        function stopDrawing() {
            isDrawing = false;
            ctx.beginPath();
        }

        function exportCanvasAsImage() {
            const image = canvas.toDataURL("image/png");
            const link = document.createElement("a");
            link.download = "canvas-image.png";
            link.href = image;
            link.click();
        }

        function getCanvasBinaryArray(canvas: HTMLCanvasElement): number[][] {
            const ctx = canvas.getContext('2d')!;
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            const data = imageData.data;
            const width = imageData.width;
            const height = imageData.height;
            const binaryArray: number[][] = [];

            for (let y = 0; y < height; y++) {
                const row: number[] = [];
                for (let x = 0; x < width; x++) {
                    const index = (y * width + x) * 4;
                    const a = data[index + 3];
                    row.push(a !== 0 ? 1 : 0);
                }
                binaryArray.push(row);
            }

            return binaryArray;
        }

        await init();
        logHelloWorld();

        canvas.addEventListener("mousedown", startDrawing);
        canvas.addEventListener("mousemove", draw);
        canvas.addEventListener("mouseup", stopDrawing);
        canvas.addEventListener("mouseout", stopDrawing);

        document.getElementById("logButton")!.addEventListener("click", () => console.log(getCanvasBinaryArray(canvas)));
        document.getElementById("addButton")!.addEventListener("click", () => console.log("added 60 and 9 from RUST to get: " + addTwoNumbers(BigInt(60), BigInt(9))));

        document.getElementById("exportButton")!.addEventListener("click", exportCanvasAsImage);
});

