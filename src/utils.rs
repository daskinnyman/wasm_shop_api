use gloo::net::http::Request;

pub async fn list_products() -> String {
    let url = "https://fakestoreapi.com/products";
    let resp = Request::get(url).send().await.unwrap();
    resp.text().await.unwrap()
}

pub async fn get_product(product_id:i32) -> String {
    let url = format!("https://fakestoreapi.com/products/{}", product_id);
    let resp = Request::get(&url).send().await.unwrap();
    resp.text().await.unwrap()
}
