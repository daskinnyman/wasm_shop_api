import * as wasm from "wasm-test";

async function allProduct() {
    const res = await wasm.get_all_products();
    const data = JSON.parse(res);
    console.log("get_all_products", data)
}

async function productById() {
    const res = await wasm.get_product_by_id(1);
    const data = JSON.parse(res);
    console.log("get_product_by_id", data)
}


allProduct();
productById();
