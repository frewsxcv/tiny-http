use std::path::Path;
use std::fs;

extern crate tiny_http;

fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension() {
        None => return "text/plain",
        Some(e) => e
    };

    match extension.to_str().unwrap() {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        _ => "text/plain; charset=utf8"
    }
}

fn main() {
    let server = tiny_http::ServerBuilder::new().with_random_port().build().unwrap();
    let port = server.get_server_addr().port();
    println!("Now listening on port {}", port);

    loop {
        let rq = match server.recv() {
            Ok(rq) => rq,
            Err(_) => break
        };

        println!("{:?}", rq);

        let url = rq.get_url().to_string();
        let path = Path::new(&url);
        let file = fs::File::open(&path);

        if file.is_ok() {
            let response = tiny_http::Response::from_file(file.unwrap());

            let response = response.with_header(
                tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: get_content_type(&path).to_string(),
                }
            );

            rq.respond(response);

        } else {
            let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
            rq.respond(rep);
        }
    }
}
