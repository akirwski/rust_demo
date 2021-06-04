use dog::dog_client::DogClient;
use dog::NewDogRequest;
use rand::Rng;
mod dog;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051").connect().await?;

    let mut client = DogClient::new(channel);

    let mut rng = rand::thread_rng();
    let request = tonic::Request::new(NewDogRequest{ breed:rng.gen_range(0..3)}, );

    let response = client.send(request).await?.into_inner();
    println!("Response = {:?}", response);
    Ok(())
}
