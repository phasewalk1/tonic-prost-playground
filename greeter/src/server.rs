use tonic::{transport::Server, Request, Response, Status};

// Import protobuf codegen
use pb::hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use pb::tasks::tasks_client::TasksClient;

// Organize pb code into a module
pub mod pb {
    pub use hello_world::*;

    pub mod hello_world {
        tonic::include_proto!("helloworld");
    }
    pub mod tasks {
        tonic::include_proto!("tasks");
    }
}

/// A very simple greeter service that greets a user by name.
#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    // impl the various Greeter service RPC methods here
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        // Create a response message for the client of the Greeter service
        let reply = pb::hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        // Nested RPC - establish a TasksClient and call the GetTasks RPC
        // This is where the two gRPC services are linked together in this example
        let mut client = TasksClient::connect("http://[::1]:3001").await.unwrap();
        let request = tonic::Request::new(());
        // Make an RPC call to the Tasks service
        let response = client.get_tasks(request).await.unwrap();
        println!("RESPONSE FROM [localhost:3001]={:?}", response);

        Ok(Response::new(reply))
    }
}

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
