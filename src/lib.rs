
#![allow(unused_variables)]
fn main() {
    use wasm_bindgen::prelude::*;

    // Called by our JS entry point to run the example
    #[wasm_bindgen(start)]
    pub fn rust_in_peace() -> Result<(), JsValue> {
        // Use `web_sys`'s global `window` function to get a handle on the global
        // window object.
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        // Manufacture the element we're gonna append
        let iframe = document.create_element("iframe")?;
        iframe.set_attribute("width", "560");
        iframe.set_attribute("height", "315");
        iframe.set_attribute("src", "https://www.youtube.com/embed/Mmbo4jQBKVU");

        let h2 = document.create_element("h2")?;
        h2.set_inner_html("Rust In Peace made by Rust!");

        body.append_child(&h2)?;
        body.append_child(&iframe)?;

       Ok(())
    }

}
