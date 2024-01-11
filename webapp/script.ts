import init, {addTwoNumbers, getPowers, logHelloWorld} from "../build_artifacts/wasm/recognize.js";

document.addEventListener("DOMContentLoaded", async function () {
    const canvas = document.getElementById("drawingCanvas") as HTMLCanvasElement;
    const ctx = canvas.getContext("2d")!;
    let isDrawing = false;

    function startDrawing(event: MouseEvent) {
        isDrawing = true;
        draw(event); }

        function clear() {
            ctx.clearRect(0, 0, canvas.width, canvas.height);

        }

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

        // get the activated indices out
        // TODO resolution?
        function getCanvasUsedIndices(canvas: HTMLCanvasElement): number[] {
            const ctx = canvas.getContext('2d')!;
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            const data = imageData.data;
            const width = imageData.width;
            const height = imageData.height;
            const binaryArray: number[] = [];

            for (let y = 0; y < height; y++) {
                for (let x = 0; x < width; x++) {
                    const index = (y * width + x) * 4;
                    const a = data[index + 3];
                    if (a !== 0) {
                        binaryArray.push(x)
                        binaryArray.push(y)
                    }
                }
            }
            return binaryArray;
        }

        await init();
        logHelloWorld();

        canvas.addEventListener("mousedown", startDrawing);
        canvas.addEventListener("mousemove", draw);
        canvas.addEventListener("mouseup", stopDrawing);
        canvas.addEventListener("mouseout", stopDrawing);

        document.getElementById("logButton")!.addEventListener("click", () => console.log(getCanvasUsedIndices(canvas)));
        document.getElementById("addButton")!.addEventListener("click", () => console.log("added 60 and 9 from RUST to get: " + addTwoNumbers(BigInt(60), BigInt(9))));

        document.getElementById("clearButton")!.addEventListener("click", clear);
});

