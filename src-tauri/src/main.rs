// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
// use std::error::Error;
use std::{fs, str::FromStr, sync::Mutex};
// use tauri::Manager;

// struct MyState {
//     file_content: String,
//     line_index: usize, // morph_iter: Box<dyn Iterator<Item = Morph>>,
// }

static KNOWN_FILE: Mutex<String> = Mutex::new(String::new());

#[derive(Serialize, Default)]
struct Morph {
    lemma: String,
    inflection: String,
}

struct ParseMorphError;

impl FromStr for Morph {
    type Err = ParseMorphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut morph = s.split(",");
        Ok(Morph {
            lemma: morph.next().ok_or(ParseMorphError)?.to_string(),
            inflection: morph.next().ok_or(ParseMorphError)?.to_string(),
        })
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            let file_content = fs::read_to_string("../public/es-freq.csv")?;
            *KNOWN_FILE.lock()? = file_content.to_string();
            
            // let morph_list = file_content
            //     .split("\n")
            //     .skip(1)
            //     .map(|text| Morph::from_str(text).unwrap_or_default());

            // app.manage(morph_list);
            // app.manage(MyState {
            //     file_content: file_content,
            //     line_index: 1,
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![next_morph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn next_morph() -> Option<Morph> {
    // let file_content = fs::read_to_string("public/es.freq.csv").unwrap_or_default();
    let file_content = KNOWN_FILE.lock().ok()?;
    println!("asdasdsa");
    println!("{file_content}");
    // let mut morph_list = state
    //     .file_content
    //     .split("\n")
    //     .skip(state.line_index)
    //     .map(|text| {
    //         let mut morph = text.split(",");
    //         Morph {
    //             lemma: morph.next().unwrap_or_default().to_string(),
    //             inflection: morph.next().unwrap_or_default().to_string(),
    //         }
    //     });
    Some(Morph::default())

    // state.line_index += 1;
    // morph_list.next()
}

// -> Option<Morph>
// -> Result<(), String>
// -> Result<Morph, Box<dyn Error>>
