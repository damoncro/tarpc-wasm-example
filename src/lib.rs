use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub async fn ping() -> Result<String, Error> {
    let resp = reqwest::Client::new()
        .get("http://localhost:3000")
        .send()
        .await?
        .text()
        .await?;
    println!("resp: {}", resp);
    Ok(resp)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error: {0}")]
    RequestError(#[from] reqwest::Error),
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}


#[wasm_bindgen]
pub async fn key_gen() -> Result<String, Error> {
    let resp = reqwest::Client::new()
        .get("http://localhost:3000/key_gen")
        .send()
        .await?
        .text()
        .await?;
    println!("resp: {}", resp);
    Ok(resp)
}
