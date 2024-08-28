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

struct ApiKey(String);

/* ======================== *\
    #Main
\* ======================== */

fn main() {
    // Grab Known Morphs
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

    // Get the Translation for the first word
    let deepl_api_key = read_secret_file().DEEPL_API_KEY;
    if let Some(m) = morph_list.get(0) {
        let mut morph = m.lock().unwrap();
        tauri::async_runtime::block_on(async {
            morph.english = deepl::translate(&deepl_api_key, &morph.inflection).await;
        });
    }

    tauri::Builder::default()
        .manage(ApiKey(deepl_api_key))
        .manage(Store {
            morph_list: morph_list,
            index: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![next_morph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

/* ======================== *\
    #Commands
\* ======================== */

#[tauri::command]
fn next_morph(state: tauri::State<Store>, app_handle: tauri::AppHandle) -> Option<Morph> {
    // Start Child Thread translate the next untranslated word
    thread::spawn(move || {
        let ApiKey(key) = app_handle.state::<ApiKey>().inner();
        let state = app_handle.state::<Store>();

        // find the next non-translated word
        let found = state
            .morph_list
            .iter()
            .filter_map(|m| m.lock().ok())
            .find(|m| m.english == None);

        // Request Translations the next word
        if let Some(mut m) = found {
            tauri::async_runtime::block_on(async {
                let translation = deepl::translate(&key, &m.inflection).await;
                // println!("next: {translation:?}");
                m.english = translation;
            });
        }
    });

    // Send back the next morph with a translation
    let mut index = state.index.lock().ok()?;
    let next = state.morph_list.get(*index)?.lock().ok()?.clone();
    *index += 1;
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

fn read_secret_file() -> SecretFile {
    let secret_file =
        fs::read_to_string("../secret.json").expect("Failed to find file 'secret.json'");
    let json =
        serde_json::from_str::<SecretFile>(&secret_file).expect("Failed to parse 'secret.json'");
    json
}
