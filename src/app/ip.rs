use actix_web::{get, web::ServiceConfig, HttpRequest, HttpResponse, Responder};

pub fn register_routes(cfg: &mut ServiceConfig) {
    cfg.service(ip);

    log::info!("ip routes loaded");
}

#[get("/ip")]
async fn ip(req: HttpRequest) -> impl Responder {
    let peer_addr = match req.peer_addr() {
        Some(ip) => ip.ip().to_string(),
        None => "Unknown".to_string(),
    };

    let conn_info = req.connection_info();
    let real_remote_addr = conn_info.realip_remote_addr().unwrap_or("unknown");

    log::debug!("Peer addr : {peer_addr}");
    log::debug!("Real ip remote addr : {real_remote_addr}");

    HttpResponse::Ok().body(format!("<html><head><title>What's your ip</title></head><body><h1>What's your ip ?</h1>Your ip address is {real_remote_addr}</body></html>"))
}
