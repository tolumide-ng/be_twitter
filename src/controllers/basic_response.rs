use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())


}