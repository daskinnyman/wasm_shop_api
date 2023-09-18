mod utils;

use utils::{get_product, list_products};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn get_all_products() -> String {
    let products = list_products().await;
    return products;
}

#[wasm_bindgen]
pub async fn get_product_by_id(product_id: i32) -> String {
    let products = get_product(product_id).await;
    return products;
}
