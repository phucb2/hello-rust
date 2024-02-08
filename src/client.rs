use myapp::{InviteReponse, InviteRequest};
use myapp::chat_client::{ChatClient};
pub mod myapp {
    tonic::include_proto!("myapp");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(InviteRequest {
        name: "Tonic".into(),
    });
    let res = client.send(request).await?;
    println!("RESPONSE={:?}", res);
    Ok(())
}