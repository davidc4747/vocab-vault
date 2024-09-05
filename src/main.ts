import { Morph, answerCard } from "./tauri";

/* ======================== *\
    #State
\* ======================== */

type State = {
    currMorph: Morph;
    questionState: QuestionState;
};
enum QuestionState {
    FRONT = "front",
    BACK = "back",
}

let state: State;

/* ======================== *\
    #Actions
\* ======================== */

async function answer(state: State, isCorrect: boolean): Promise<State> {
    // Mark the value as correct, or incorrect
    //      Ask for the next value
    const morph = await answerCard(isCorrect);

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
    #Init
\* ======================== */

let rawElem: Element | null,
    mainElem: Element | null,
    spanishElem: Element | null,
    englishElem: Element | null,
    showAnswerBtn: HTMLButtonElement | null,
    correctBtn: HTMLButtonElement | null,
    incorrectBtn: HTMLButtonElement | null;
window.addEventListener("DOMContentLoaded", async function (): Promise<void> {
    // Main
    mainElem = document.body.querySelector(".main");
    rawElem = document.body.querySelector(".raw");
    spanishElem = document.querySelector(".spanish");
    englishElem = document.querySelector(".english");

    // Buttons
    showAnswerBtn =
        document.querySelector<HTMLButtonElement>(".btn-show-answer");
    correctBtn = document.querySelector<HTMLButtonElement>(".btn-correct");
    incorrectBtn = document.querySelector<HTMLButtonElement>(".btn-incorrect");

    /* ------------------------ *\
        #Events
    \* ------------------------ */

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

    /* ------------------------ *\
        # Initialize State
    \* ------------------------ */

    state = await (async function (): Promise<State> {
        const currMorph = await answerCard(false);
        return {
            currMorph,
            questionState: QuestionState.FRONT,
        };
    })();
    render(state);
});

/* ======================== *\
    #Render
\* ======================== */

function render({ currMorph, questionState }: State): void {
    // NOTE: this element is just here for debugging [DC]
    // rawElem?.replaceChildren("");

    if (mainElem) mainElem.className = `main--${questionState}`;
    switch (questionState) {
        case QuestionState.FRONT:
            // Display the Morph
            spanishElem?.replaceChildren(currMorph.inflection);
            spanishElem?.classList.remove("spanish--back");

            // Hide Answer
            englishElem?.classList.remove("english--back");

            // Display the appropriate Buttons
            showAnswerBtn?.classList.remove("hidden");
            incorrectBtn?.classList.add("hidden");
            correctBtn?.classList.add("hidden");
            break;
        case QuestionState.BACK:
            spanishElem?.classList.add("spanish--back");

            // Show Answer
            if (englishElem && currMorph.english) {
                englishElem.replaceChildren(currMorph.english);
                englishElem?.classList.add("english--back");
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
