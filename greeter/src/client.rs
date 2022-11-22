use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

// @brief: Core client request handler that generates code for each request made
// @params
// - client(ident): GreeterClient
// - request(expr): HelloRequest
macro_rules! handle_client {
    ($client:ident, $request:expr) => {
        println!(
            "RESPONSE={:?}",
            $client::connect("http://[::1]:50051")
                .await
                .unwrap()
                .say_hello(tonic::Request::new($request))
                .await
                .unwrap()
        )
    };
}

impl TryFrom<String> for HelloRequest {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(HelloRequest { name: value })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let request = HelloRequest::try_from(args[1].clone())?;
    let _res = handle_client!(GreeterClient, HelloRequest::try_from(request)?);

    Ok(())
}
