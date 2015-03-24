#![feature(core)]

extern crate iron;
extern crate router;

use iron::{ Iron, IronResult, Request, Response };
use iron::status;
use router::Router;
use std::error::Error;

fn main() {
    let router = build_router();

    match Iron::new(router).http("localhost:1337") {
        Ok(_) => println!("LEADED server started."),
        Err(e) => println!("{}", Error::description(&e)),
    }
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello!")))
}

fn build_router() -> Router {
    let mut router = Router::new();
    router.get("/", handler);
    router
}
