import { invoke } from "@tauri-apps/api/tauri";

export type Morph = {
    lemma: string;
    inflection: string;
    english: string | null;
};

export async function answerCard(isCorrect: boolean): Promise<Morph> {
    const data = await invoke("answer", { isCorrect });
    return data as Morph;
}
