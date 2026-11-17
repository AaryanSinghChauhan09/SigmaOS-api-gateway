use sigmaos_api_gateway::{ApiGateway, GatewayError, Request, Response};

fn usage() -> ! {
    eprintln!("usage: sigmaos-api-gateway");
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        usage();
    }
    let mut gw = ApiGateway::new();
    gw.add_route("/health", |_| Ok(Response::ok().with_body(b"OK".to_vec())));
    let req = Request::new("GET", "/health");
    match gw.handle(req) {
        Ok(res) => println!("status {}", res.status),
        Err(e) => eprintln!("error: {e}"),
    }
}
