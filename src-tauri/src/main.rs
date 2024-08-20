// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::{fs, str::FromStr, sync::Mutex};

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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![next_morph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn next_morph() -> Option<Morph> {
    let file_content = KNOWN_FILE.lock().ok()?;
    Some(Morph::default())
}
