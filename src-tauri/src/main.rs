// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod deepl;
mod morph;

use morph::Morph;
use serde::Deserialize;
use std::str::FromStr;
use std::thread;
use std::{fs, sync::Mutex};
use tauri::Manager;

#[derive(Debug, Default)]
struct Store {
    morph_list: Vec<Mutex<Morph>>,
    index: Mutex<usize>,
}

/* ======================== *\
    #Main
\* ======================== */

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Grap Known Morphs
            let known = fs::read_to_string("../public/known_morphs-2024-08-26@12-51-37.csv")
                .expect("know_morph csv file now found")
                .split("\n")
                .skip(1)
                .filter_map(|text| Morph::from_str(text).ok())
                .map(|morph| morph.inflection)
                .collect::<Vec<String>>();

            // Grab frequency file
            let morph_list = fs::read_to_string("../public/es-freq.csv")
                .expect("frequency csv file now found")
                .split("\n")
                .skip(1)
                .filter_map(|text| Morph::from_str(text).ok())
                .filter(|morph| {
                    // make sure it's not in the "known.csv"
                    morph.lemma == morph.inflection && !known.contains(&morph.inflection)
                })
                .map(|m| Mutex::new(m.clone()))
                .collect::<Vec<Mutex<Morph>>>();

            app.manage(Store {
                morph_list,
                index: Mutex::new(0),
            });

            // Start Child Thread to request DeepL Transalations in the background
            let handle = app.app_handle();
            thread::spawn(move || {
                let state = handle.state::<Store>().inner();

                // Request Translations each morph
                let deepl_api_key = get_api_key().expect("Failed to load DeepL API key");
                state.morph_list.iter().for_each(|m| {
                    let mut morph = m.lock().unwrap();
                    tauri::async_runtime::block_on(async {
                        let translation = deepl::translate(&deepl_api_key, &morph.inflection).await;
                        println!("state: {translation:?}");
                        morph.english = translation;
                    });
                });
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
    let next = state.morph_list.get(*index)?.lock().ok()?.clone();
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
