use tonic::Request;

// Include generated gRPC code from hello.proto
pub mod hello {
    tonic::include_proto!("hello");
}

use hello::greeter_client::GreeterClient;
use hello::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to gRPC server
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // Build request with name to send
    let request = Request::new(HelloRequest {
        name: "Rustacean".into(),
    });

    // Call SayHello and get the streaming response
    let mut stream = client.say_hello(request).await?.into_inner();

    // Receive and print each message from server
    while let Some(reply) = stream.message().await? {
        println!("Got: {}", reply.message);
    }

    Ok(())
}
