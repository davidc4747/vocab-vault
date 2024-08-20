import { Morph, nextMorph } from "./tauri";

/* ======================== *\
    #State
\* ======================== */

type State = {
    currMorph: Morph;
    questionState: QuestionState;
};
enum QuestionState {
    FRONT,
    BACK,
}

let state = {
    currMorph: await nextMorph(),
    questionState: QuestionState.FRONT,
};
render(state);

/* ======================== *\
    #Actions
\* ======================== */

async function answer(state: State, isCorrect: boolean): Promise<State> {
    // TODO: Mark the value as correct, or incorrect.
    // TODO: Send it back to Rust.
    const morph = await nextMorph();

    // TODO: use DeepL to get the transaltion of the word.

    return {
        ...state,
        currMorph: morph,
        questionState: QuestionState.FRONT,
    };
}

function showAnswer(state: State): State {
    return {
        ...state,
        questionState: QuestionState.BACK,
    };
}

/* ======================== *\
    #Events
\* ======================== */

document
    .querySelector<HTMLButtonElement>(".btn-show-answer")
    ?.addEventListener("click", () => render(showAnswer(state)));

document
    .querySelector<HTMLButtonElement>(".btn-incorrect")
    ?.addEventListener("click", async () => render(await answer(state, false)));
document
    .querySelector<HTMLButtonElement>(".btn-correct")
    ?.addEventListener("click", async () => render(await answer(state, true)));

/* ======================== *\
    #Render
\* ======================== */

function render({ currMorph, questionState }: State): void {
    document.querySelector(".raw")?.replaceChildren(JSON.stringify(currMorph));
    switch (questionState) {
        case QuestionState.FRONT:
            // Display the Morph
            document
                .querySelector(".lemma")
                ?.replaceChildren(`(${currMorph.lemma})`);
            document
                .querySelector(".inflection")
                ?.replaceChildren(currMorph.inflection);

            // Hide Answer
            document.querySelector(".english")?.classList.add("hidden");

            // Display the appropriate Buttons
            document
                .querySelector(".btn-show-answer")
                ?.classList.remove("hidden");
            document.querySelector(".btn-incorrect")?.classList.add("hidden");
            document.querySelector(".btn-correct")?.classList.add("hidden");
            break;
        case QuestionState.BACK:
            // Show Answer
            document.querySelector(".english")?.classList.remove("hidden");

            // Display the appropriate Buttons
            document.querySelector(".btn-show-answer")?.classList.add("hidden");
            document
                .querySelector(".btn-incorrect")
                ?.classList.remove("hidden");
            document.querySelector(".btn-correct")?.classList.remove("hidden");
            break;

        default:
            break;
    }
}
