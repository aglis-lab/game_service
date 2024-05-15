use actix_web::*;

mod response;
mod sdp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addrs = ("127.0.0.1", 8080);
    let server = HttpServer::new(|| App::new().route("/sdp", web::post().to(sdp::sdp_service)))
        .bind(addrs)?
        .run();

    println!("\nListening on {:?}", addrs);

    server.await
}
