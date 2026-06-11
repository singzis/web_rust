mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum_array(arr: &js_sys::Array) -> u32 {
    let mut sum = 0;

    for i in 0..arr.length() {
        let val = arr.get(i).as_f64().unwrap_or(0.0) as u32;
        sum += val
    }

    sum
}

#[wasm_bindgen]
pub fn set_title(title: &str) {
    // 获取window对象
    let win = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    // 获取document对象
    let doc = match win.document() {
        Some(d) => d,
        None => return,
    };

    doc.set_title(title)
}
