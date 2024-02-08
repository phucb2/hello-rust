// use tonic::{transport::Server, Request, Response, Status};
// // use hello::say_server::{Say, SayServer};
// // use hello::{SayResponse, SayRequest};

// // defining a struct for our service
// #[derive(Default)]
// pub struct MySay {}

// // implementing rpc for service defined in .proto
// #[tonic::async_trait]
// impl Say for MySay {
// // our rpc impelemented as function
//     async fn send(&self,request:Request<SayRequest>)->Result<Response<SayResponse>,Status>{
// // returning a response as SayResponse message as defined in .proto
//         Ok(Response::new(SayResponse{
// // reading data from request which is awrapper around our SayRequest message defined in .proto
//              message:format!("hello {}",request.get_ref().name),
//         }))
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
// // defining address for our service
//     let addr = "[::1]:50051".parse().unwrap();
// // creating a service
//     let say = MySay::default();
//     println!("Server listening on {}", addr);
// // adding our service to our server.
//     Server::builder()
//         .add_service(SayServer::new(say))
//         .serve(addr)
//         .await?;
//     Ok(())
// }

// Compare this snippet from Cargo.toml:
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}
#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

pub mod myapp {
    tonic::include_proto!("myapp");
}

use myapp::{InviteReponse, InviteRequest};
use myapp::chat_server::{Chat, ChatServer};

#[derive(Debug, Default)]
pub struct MyChat {}

#[tonic::async_trait]
impl Chat for MyChat {
    async fn send(
        &self,
        request: Request<InviteRequest>,
    ) -> Result<Response<InviteReponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = myapp::InviteReponse {
            other: format!("Hello {}!", request.into_inner().name),
            context: 1,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    let chat = MyChat::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(ChatServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}