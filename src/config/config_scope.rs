use crate::routes::healthz::check_health;
use crate::routes::me::me;
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1").service(check_health).service(me);
    conf.service(scope);
}
