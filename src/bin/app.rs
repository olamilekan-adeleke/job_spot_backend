use actix_rt;
use actix_web::web;

#[path = "../cores/mod.rs"]
mod cores;

#[actix_rt::main]
async fn main() -> io::Result<()> {}
