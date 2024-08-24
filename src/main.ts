import { Morph, nextMorph, translate } from "./tauri";

/* ======================== *\
    #State
\* ======================== */

type State = {
    currMorph: Morph;
    english: string;
    questionState: QuestionState;
};
enum QuestionState {
    FRONT,
    BACK,
}

let state: State = await (async function (): Promise<State> {
    const currMorph = await nextMorph();
    return {
        currMorph,
        english: (await translate(currMorph.inflection)) ?? "",
        questionState: QuestionState.FRONT,
    };
})();
render(state);

/* ======================== *\
    #Actions
\* ======================== */

async function answer(state: State, isCorrect: boolean): Promise<State> {
    // TODO: Mark the value as correct, or incorrect.
    // TODO: Send it back to Rust.
    const morph = await nextMorph();

    // TODO: use DeepL to get the transaltion of the word.
    const english = (await translate(morph.inflection)) ?? "";

    return {
        ...state,
        currMorph: morph,
        english,
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

let rawElem: Element | null,
    lemmaElem: Element | null,
    inflectionElem: Element | null,
    englishElem: Element | null,
    showAnswerBtn: HTMLButtonElement | null,
    correctBtn: HTMLButtonElement | null,
    incorrectBtn: HTMLButtonElement | null;
window.addEventListener("DOMContentLoaded", function (): void {
    // Main
    rawElem = document.body.querySelector(".raw");
    lemmaElem = document.querySelector(".lemma");
    inflectionElem = document.querySelector(".inflection");
    englishElem = document.querySelector(".english");

    // Buttons
    showAnswerBtn =
        document.querySelector<HTMLButtonElement>(".btn-show-answer");
    correctBtn = document.querySelector<HTMLButtonElement>(".btn-incorrect");
    incorrectBtn = document.querySelector<HTMLButtonElement>(".btn-correct");

    // Events
    showAnswerBtn?.addEventListener("click", () => {
        state = showAnswer(state);
        render(state);
    });

    incorrectBtn?.addEventListener("click", async () => {
        state = await answer(state, false);
        render(state);
    });
    correctBtn?.addEventListener("click", async () => {
        state = await answer(state, true);
        render(state);
    });
});

/* ======================== *\
    #Render
\* ======================== */

function render({ currMorph, questionState, ...state }: State): void {
    rawElem?.replaceChildren(JSON.stringify(currMorph));
    switch (questionState) {
        case QuestionState.FRONT:
            // Display the Morph
            lemmaElem?.replaceChildren(`(${currMorph.lemma})`);
            inflectionElem?.replaceChildren(currMorph.inflection);

            // Hide Answer
            englishElem?.classList.add("hidden");

            // Display the appropriate Buttons
            showAnswerBtn?.classList.remove("hidden");
            incorrectBtn?.classList.add("hidden");
            correctBtn?.classList.add("hidden");
            break;
        case QuestionState.BACK:
            // Show Answer
            if (englishElem) {
                englishElem.replaceChildren(state.english);
                englishElem.classList.remove("hidden");
            }

            // Display the appropriate Buttons
            showAnswerBtn?.classList.add("hidden");
            incorrectBtn?.classList.remove("hidden");
            correctBtn?.classList.remove("hidden");
            break;

        default:
            break;
    }
}
