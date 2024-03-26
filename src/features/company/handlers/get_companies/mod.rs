use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{
    cores::AppState, feature::company::db_access::get_all_companies_db::get_all_companies,
    map_response, BaseError,
};

#[derive(Debug, Deserialize)]
pub struct GetCompanyQuery {
    pub page_size: Option<i64>,
    pub last_company_name: Option<String>,
}

pub async fn get_compaines(
    app_state: web::Data<AppState>,
    query: web::Query<GetCompanyQuery>,
) -> Result<HttpResponse, BaseError> {
    let page_size = query.0.page_size.unwrap_or(10);
    let last_company_name = query.0.last_company_name;

    let company_list = get_all_companies(&app_state.db, page_size, last_company_name).await?;

    let msg = "Fetched all companies sucessfully!".to_string();
    Ok(map_response(&company_list, &msg))
}
