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

let state: State = await init();
render(state);

/* ======================== *\
    #Actions
\* ======================== */

async function init(): Promise<State> {
    const currMorph = await nextMorph();
    return {
        currMorph,
        english: (await translate(currMorph.inflection)) ?? "",
        questionState: QuestionState.FRONT,
    };
}

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

document
    .querySelector<HTMLButtonElement>(".btn-show-answer")
    ?.addEventListener("click", () => {
        state = showAnswer(state);
        render(state);
    });

document
    .querySelector<HTMLButtonElement>(".btn-incorrect")
    ?.addEventListener("click", async () => {
        state = await answer(state, false);
        render(state);
    });
document
    .querySelector<HTMLButtonElement>(".btn-correct")
    ?.addEventListener("click", async () => {
        state = await answer(state, true);
        render(state);
    });

/* ======================== *\
    #Render
\* ======================== */

function render({ currMorph, questionState, ...state }: State): void {
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
            const englishElm = document.querySelector(".english");
            if (englishElm) {
                englishElm.replaceChildren(state.english);
                englishElm.classList.remove("hidden");
            }

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
