use axum::Json;
use http::StatusCode;
use crate::transport::transport::{MenuItem, MenuRequest, MenuResponse};
#[utoipa::path(
    post,
    path = "/menu",
    responses(
        (status = 200, description = "Menu found successfully", body = MenuResponse),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
    request_body(
         content_type = "application/json",
         content = MenuRequest,
         description = "A description "
    )
)]
pub async fn get_menu(Json(payload): Json<MenuRequest>) -> Result<Json<MenuResponse>, (StatusCode, Json<String>)> {
    let item = MenuItem{name: Option::from("testName".to_string()), price: 0.0 };
    println!("{:?}", payload);
    Ok(Json(MenuResponse{items: vec![item]}))
}



