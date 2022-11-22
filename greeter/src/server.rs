use tonic::{transport::Server, Request, Response, Status};

// Import protobuf codegen
use pb::hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use pb::tasks::tasks_client::TasksClient;

#[doc = "Protobuf codegen"]
pub mod pb {
    pub use hello_world::*;

    pub mod hello_world {
        tonic::include_proto!("helloworld");
    }
    pub mod tasks {
        tonic::include_proto!("tasks");
    }
}

// @brief: Greeter service impl
#[derive(Debug, Default)]
#[doc = "A very simple greeter service that greets a user by name."]
pub struct MyGreeter {}

// @brief: Implement the RPC methods for Greeter service
#[tonic::async_trait]
impl Greeter for MyGreeter {
    // @params: Request<HelloRequest> - request from client
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        // Create a response message for the client of the Greeter service
        let reply = pb::hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        // In production, the Tasks service would not be running on the same server.
        let mut client = TasksClient::connect("http://[::1]:3001").await.unwrap();
        let request = tonic::Request::new(());
        // Make an RPC call to the Tasks service
        let response = client.get_tasks(request).await.unwrap();
        println!("RESPONSE FROM [localhost:3001]={:?}", response);

        Ok(Response::new(reply))
    }
}

// @brief: Main server entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
