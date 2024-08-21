import { invoke } from "@tauri-apps/api/tauri";

export type Morph = {
    lemma: string;
    inflection: string;
};

export async function nextMorph(): Promise<Morph> {
    const data = await invoke("next_morph");
    return data as Morph;
}

export async function translate(word: string): Promise<string> {
    const data: string = await invoke("translate", { word });
    return data;
}