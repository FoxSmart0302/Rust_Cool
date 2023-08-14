use actix_web::guard::GuardContext;
use rs_models::env;

pub fn api_key_middleware(ctx: &GuardContext) -> bool {
    let header = ctx.head().headers().get("api_key");
    if header.is_none() {
        return false;
    }
    let header = header.unwrap().to_str();
    if header.is_err() {
        return false;
    }
    let header = header.unwrap();

    header == env("RS_XLSX_API_KEY")
}
