import { invoke } from "@tauri-apps/api/tauri";

export type Morph = {
    lemma: string;
    inflection: string;
    english: string | null;
};

export async function nextMorph(): Promise<Morph> {
    const data = await invoke("next_morph");
    return data as Morph;
}
