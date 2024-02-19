use myapp::{InviteReponse, InviteRequest};
use myapp::chat_client::{ChatClient};
use open_feature::provider::NoOpProvider;
use open_feature::OpenFeature;
pub mod myapp {
    tonic::include_proto!("myapp");
}

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Acquire an OpenFeature API instance.
    // Note the `await` call here because asynchronous lock is used to
    // guarantee thread safety.
    let mut api = OpenFeature::singleton_mut().await;

    // Configure a provider.
    // By default [`NoOpProvider`] is used.
    api.set_provider(NoOpProvider::default()).await;

    // create a client
    let client = api.create_client();

    // get a bool flag value
    let is_feature_enabled = client
        .get_bool_value("v2_enabled", None, None)
        .await
        .unwrap_or(false);

    println!("v2_enabled: {}", is_feature_enabled);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = ChatClient::connect("http://[::1]:50051").await?;
    // let request = tonic::Request::new(InviteRequest {
    //     name: "Tonic".into(),
    // });
    // let res = client.send(request).await?;
    // println!("RESPONSE={:?}", res);

    // example
    example().await?;

    Ok(())
}