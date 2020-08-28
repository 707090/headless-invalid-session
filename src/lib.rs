use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn print() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn all_good() {
        assert!(true, "all is well");
    }
}
