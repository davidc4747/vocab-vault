use serde::Deserialize;
use serde_json::json;
use std::error::Error;

#[derive(Deserialize)]
struct DeepLResponse {
    translations: Vec<DeepLTranslation>,
}
#[derive(Deserialize)]
#[allow(dead_code)]
struct DeepLTranslation {
    detected_source_language: String,
    text: String,
}

pub async fn translate(deepl_api_key: &str, word: &str) -> Option<String> {
    // Send the Request
    let client = reqwest::Client::new();
    let res = client
        .post("https://api-free.deepl.com/v2/translate")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("DeepL-Auth-Key {deepl_api_key}"))
        // .body(body)
        .json(&json!({
            "source_lang": "ES",
            "target_lang": "EN",
            "text": [word],
        }))
        .send()
        .await
        .ok()?;

    let translations = res
        .json::<DeepLResponse>()
        .await
        .ok()?
        .translations
        .pop()?
        .text;

    Some(translations)
}

pub async fn _translate_multiple(
    deepl_api_key: &str,
    words: &[&str],
) -> Result<Vec<String>, Box<dyn Error>> {
    let client = reqwest::Client::new()
        .post("https://api-free.deepl.com/v2/translate")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("DeepL-Auth-Key {deepl_api_key}"))
        .json(&json!({
            "source_lang": "ES",
            "target_lang": "EN",
            "text": words
        }));

    let translations = client
        .send()
        .await?
        .json::<DeepLResponse>()
        .await?
        .translations
        .iter()
        .map(|t| t.text.clone())
        .collect();

    Ok(translations)
}
