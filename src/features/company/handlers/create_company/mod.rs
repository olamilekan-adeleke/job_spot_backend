use actix_web::{web, HttpResponse};

use crate::{
    cores::AppState,
    feature::company::{
        db_access::create_company_db::create_company_db, models::create_new_company::CreateCompany,
    },
    map_response, BaseError,
};

pub async fn create_company_handler(
    app_state: web::Data<AppState>,
    data: web::Json<CreateCompany>,
    // token_clamis: Option<web::ReqData<TokenClaims>>,
) -> Result<HttpResponse, BaseError> {
    let _ = create_company_db(&app_state.db, data.0).await?;

    let msg = "Company created Successfully!".to_string();
    Ok(map_response(&{}, &msg))
}
