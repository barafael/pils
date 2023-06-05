use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[must_use]
pub fn process_str(line: &str) -> String {
    pils::process_str(line)
}

#[wasm_bindgen]
#[must_use]
pub fn help_text() -> String {
    pils::help::HELP_TEXT.to_string()
}

#[wasm_bindgen]
#[must_use]
pub fn get_env_json() -> String {
    pils::get_env_json()
}

#[wasm_bindgen]
#[must_use]
pub fn get_env_tuples() -> String {
    pils::get_env_tuples()
}

#[wasm_bindgen]
#[must_use]
pub fn get_example_environment() -> String {
    pils::get_example_environment()
}
