use askama::Template;
use askama_axum::IntoResponse as AskamaIntoResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app: _ = Router::new()
        .route("/", get(get_root))
        .route("/lc3b_bg.wasm", get(get_lc3b_wasm))
        .route("/lc3b.js", get(get_lc3b_js));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_root() -> impl IntoResponse {
    IndexTemplate {}.into_response()
}

const LC3B_WASM_BYTES: &'static [u8] = include_bytes!("../../../lc3b/pkg/lc3b_bg.wasm");

async fn get_lc3b_wasm() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("content-type", "application/wasm")],
        LC3B_WASM_BYTES,
    )
}

const LC3B_JS_BYTES: &'static [u8] = include_bytes!("../../../lc3b/pkg/lc3b.js");

async fn get_lc3b_js() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("content-type", "application/javascript")],
        LC3B_JS_BYTES,
    )
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}
