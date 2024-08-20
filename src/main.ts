import { nextMorph } from "./tauri";

const nextBtn = document.querySelector<HTMLButtonElement>(".btn-next");
nextBtn?.addEventListener("click", async function name(): Promise<void> {
    const data = await nextMorph();
    console.log(data);

    const elem = document.querySelector(".output");
    if (elem) {
        elem.textContent = JSON.stringify(data);
    }
});
nextBtn?.click();
