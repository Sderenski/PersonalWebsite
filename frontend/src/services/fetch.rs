use {
    wasm_bindgen::{JsCast, JsValue},
    wasm_bindgen_features::JsFuture,
    web_sys::{Request, RequestInit, RequestMode, RequestRedirect, Response},
    yew::web_sys,
};

// These fetch functions are a Rust API for the browser's native fetch API.
// Uses wasm-bindgen crate to translate JS types to Rust types and vice versa

pub enum Method {
    GET,
}

pub async fn fetch(url: String, method: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(&method);
    opts.mode(RequestMode::Cors);
    opts.redirect(RequestRedirect::Follow);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?;
    request.headers().set("Content-Type", "application/json")?;
    request.headers().set(
        "Access-Control-Request-Headers",
        "Content-Type, Authorization",
    )?;
    request
        .headers()
        .set("Access-Control-Request-Method", &method)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert a JS Promise into a Rust Future
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

// JsFuture that converts JS Promises into Rust futures that will then be processed
// by a `handle_future` function 

// Wrap the fetch function into another one that will call the first with the proper arguments

pub struct Fetch();

impl Fetch {
    async fn fetch(
        url: String,
        method: Method,
    ) -> Result<JsValue, JsValue> {
        let method = match method {
            Method::GET => "GET",
        };
        fetch(url, method.to_string()).await
    }

    pub async fn get(url: String) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::GET).await
    }
}