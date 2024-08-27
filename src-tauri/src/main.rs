// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod deepl;
mod morph;

use morph::Morph;
use serde::Deserialize;
use std::str::FromStr;
use std::{fs, sync::Mutex};
use tauri::Manager;

#[derive(Default)]
struct Store {
    morph_list: Vec<Morph>,
    index: Mutex<usize>,
}

/* ======================== *\
    #Main
\* ======================== */

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Grap Known Morphs
            let known = fs::read_to_string("../public/known_morphs-2024-08-26@12-51-37.csv")?
                .split("\n")
                .skip(1)
                .filter_map(|text| Morph::from_str(text).ok())
                .map(|morph| morph.inflection)
                .collect::<Vec<String>>();

            // Grab frequency file
            let deepl_api_key = get_api_key().expect("Failed to load DeepL API key");
            let mut morph_list = fs::read_to_string("../public/es-freq.csv")?
                .split("\n")
                .skip(1)
                .filter_map(|text| Morph::from_str(text).ok())
                .filter(|morph| {
                    // make sure it's not in the "known.csv"
                    morph.lemma == morph.inflection && !known.contains(&morph.inflection)
                })
                .collect::<Vec<Morph>>();

            // Request Translations for First few morphs
            morph_list.iter_mut().take(2).for_each(|morph| {
                let mut output = None;
                tauri::async_runtime::block_on(async {
                    output = deepl::translate(&deepl_api_key, &morph.inflection).await;
                });
                morph.english = output
            });

            // Initialize State
            app.manage(Store {
                morph_list,
                index: Mutex::new(0),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![next_morph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/* ======================== *\
    #Commands
\* ======================== */

#[tauri::command]
fn next_morph(state: tauri::State<Store>) -> Option<Morph> {
    let mut index = state.index.lock().unwrap();
    *index += 1;
    let next = state.morph_list.get(*index)?.clone();
    Some(next)
}

/* ======================== *\
    #Utils
\* ======================== */

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct SecretFile {
    DEEPL_API_KEY: String,
}

fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    let secret_file = fs::read_to_string("../secret.json")?;
    let json = serde_json::from_str::<SecretFile>(&secret_file)?;
    Ok(json.DEEPL_API_KEY)
}
