use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn print(input : &str) -> String {
    let result = rsjson::parse(input);
    match result {
        Ok(value) => value.to_string(),
        Err(err) => format!("{}",err)
    }
}