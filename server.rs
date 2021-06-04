use tonic::{transport::Server, Request, Response, Status};
use dog::dog_server::Dog;//this came from service Dog
use dog::dog_server::DogServer;
use dog::{NewDogRequest, NewDogResponse};


mod dog;//include a package called dog

#[derive(Default)]
pub struct DogServiceImple {}

#[tonic::async_trait]
impl Dog for DogServiceImple {
    //Status returns error message
    async fn send(&self, request:Request<NewDogRequest>) -> Result<Response<NewDogResponse>, Status> {
        // let sex = request.get_ref().sex;
        // let breed = request.get_ref().breed;
        // let female_male : String;
        // let breed_name : String;
        //
        // if sex {
        //     female_male = String::from("female");
        // } else {
        //     female_male = String::from("male");
        // }
        //
        // let breed_name = match breed {
        //     0 => String::from("Bulldog"),
        //     1 => String::from("Pooldle"),
        //     2 => String::from("Chihuahua"),
        // };

        Ok(Response::new(NewDogResponse{ name: String::from("pochi"), age: 0, sex: false, breed: request.get_ref().breed,})),
        Err(Status::unauthenticated("Failded to send a response")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse().unwrap();
    let newdog = DogServiceImple::default();
    println!("Server listening on {}", address);
    Server::builder().add_service(DogServer::new(newdog)).serve(address).await?;
    Ok(())
}
