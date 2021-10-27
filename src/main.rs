#[macro_use]
extern crate derive_builder;

use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

mod domain;
mod usecase;

pub mod hello_world {
    // import generated gRPC code
    tonic::include_proto!("helloworld");
}

#[derive(Default)] // add default() method
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!(
            "request: {:?} (from {:?})",
            req.get_ref(),
            req.remote_addr()
        );

        let reply = hello_world::HelloReply {
            message: format!("Hello, {}!", req.get_ref().name),
        };

        println!("response: {:?}", reply);

        // return response
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
