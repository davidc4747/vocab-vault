// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::{fs, str::FromStr, sync::Mutex};
use tauri::Manager;

#[derive(Default)]
struct Store {
    unknown_file_content: String,
    index: Mutex<usize>,
}

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
        .setup(|app| {
            let file_content = fs::read_to_string("../public/es-freq.csv")?;

            app.manage(Store {
                unknown_file_content: file_content,
                index: Mutex::new(0),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![next_morph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn next_morph(state: tauri::State<Store>) -> Option<Morph> {
    let Store {
        unknown_file_content,
        index,
    } = state.inner();

    let mut num = index.lock().unwrap();
    *num += 1;

    let mut morph_list = unknown_file_content
        .split("\n")
        .skip(*num)
        .map(|text| Morph::from_str(text).unwrap_or_default());

    morph_list.next()
}
