use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse, Json};
use serde_json::json;

pub async fn custom_fn_middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("custom_fn_middleware work!");

    println!(
        "reqURL: {:?} \nquery: {:?} \nmethod: {:?} \nheaders: {:?} \nbody: {:?}",
        req.uri(),
        req.uri().query(),
        req.method(),
        req.headers().values(),
        req.body()
    );

    let (parts, body) = req.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    let body_json = Json(json!(body_str));
    // let body_struct: Message = serde_json::from_value(json!(body_str)).unwrap();

    println!(
        "body bytes: {:?} \nbody str: {:?} \nbody json: {:?}",
        body_bytes, body_str, body_json
    );

    let req = Request::from_parts(parts, body_bytes.into());

    let response = next.run(req).await;

    // println!("response: {:?}", response.body());
    Ok(response)
}
