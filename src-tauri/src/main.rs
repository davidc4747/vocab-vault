// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod deepl;
mod morph;

use morph::Morph;
use serde::Deserialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use std::{fs, sync::Mutex};
use tauri::http::status;
use tauri::Manager;

#[derive(Debug, Default)]
struct Store {
    morph_list: Vec<Mutex<Morph>>,
    index: Mutex<usize>,
    known_count: usize,
    total_count: usize,
}

struct ApiKey(String);

/* ======================== *\
    #Main
\* ======================== */

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Path to local data
            let data_dir = app.path_resolver().app_data_dir().unwrap_or_default();

            // Grab Known Morphs
            let known = csv_to_morphlist(
                &fs::read_to_string(data_dir.join("known.csv")).unwrap_or_default(),
            );
            let unknown = csv_to_morphlist(
                &fs::read_to_string(data_dir.join("unknown.csv")).unwrap_or_default(),
            );
            println!("{known:?}");

            // Grab frequency file, Place Morphs into Mutex
            let morph_list = fs::read_to_string("../public/es-freq.csv")
                .expect("frequency csv file now found")
                .split("\n")
                .skip(1)
                .filter_map(|text| Morph::from_str(text).ok())
                .filter(|morph| {
                    // make sure it's not in the "known.csv"
                    morph.lemma == morph.inflection
                        && !known.contains(&morph.inflection)
                        && !unknown.contains(&morph.inflection)
                })
                .map(|m| Mutex::new(m.clone()))
                .collect::<Vec<Mutex<Morph>>>();
            let total_count = morph_list.len();

            // Get the Translation for the first word
            let deepl_api_key = read_secret_file().DEEPL_API_KEY;
            if let Some(m) = morph_list.get(0) {
                let mut morph = m.lock().unwrap();
                tauri::async_runtime::block_on(async {
                    morph.english = deepl::translate(&deepl_api_key, &morph.inflection).await;
                });
            }

            // Initalize State
            app.manage(ApiKey(deepl_api_key));
            app.manage(Store {
                morph_list: morph_list,
                index: Mutex::new(0),
                known_count: known.len(),
                total_count,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![answer, get_known_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/* ======================== *\
    #Commands
\* ======================== */

#[tauri::command]
fn answer(
    state: tauri::State<Store>,
    app_handle: tauri::AppHandle,
    is_correct: bool,
) -> Option<Morph> {
    // Start Child Thread translate the next untranslated word in the background
    let handle = app_handle.app_handle();
    thread::spawn(move || {
        let ApiKey(key) = handle.state::<ApiKey>().inner();
        let state = handle.state::<Store>();

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

    // Save the morph to the correct file based on how they answered
    let mut index = state.index.lock().ok()?;
    if *index > 0 {
        let previous_morph = state.morph_list.get(*index - 1)?.lock().ok()?.clone();
        let app_data_dir = app_handle.path_resolver().app_data_dir()?;
        let filename = if is_correct {
            "known.csv"
        } else {
            "unknown.csv"
        };
        update_morph_file(app_data_dir, filename, previous_morph);
    }

    // Give them anther morph to review
    let next_morph = state.morph_list.get(*index)?.lock().ok()?.clone();
    *index += 1;
    Some(next_morph)
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

fn csv_to_morphlist(file_content: &str) -> Vec<String> {
    file_content
        .split("\n")
        .skip(1)
        .filter_map(|text| Morph::from_str(text).ok())
        .map(|morph| morph.inflection)
        .collect::<Vec<String>>()
}

fn update_morph_file(dir: PathBuf, filename: &str, morph: Morph) -> () {
    let file = dir.join(filename);
    let path_string = file.clone().into_os_string().into_string().unwrap();

    // Make sure the directory is there
    fs::create_dir_all(dir).expect(&format!("Failed to create directory '{path_string}'"));

    // Create the csv File if it doesn't exist
    if !file.exists() {
        fs::write(&file, "Morph-lemma,Morph-inflection\n")
            .expect(&format!("Unable to create file '{path_string}'"));
    }

    // append the morph to the file
    let mut file = OpenOptions::new()
        .append(true)
        .open(&file)
        .expect("Unable to open 'known.csv'");
    writeln!(file, "{},{}", morph.lemma, morph.inflection)
        .expect(&format!("Unable to append to file '{path_string}'"));
}
