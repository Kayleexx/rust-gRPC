use tonic::{transport::Server, Request, Response, Status};
use futures_core::Stream;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc; // async multi-producer, single-consumer channel
use std::pin::Pin;

// Generated code from hello.proto will be included here
pub mod hello {
    tonic::include_proto!("hello"); 
}

// Bring server traits and message types into scope
use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};

// Struct to hold our server logic (no fields for now)
#[derive(Default)]
struct MyGreeter;

// Implement the gRPC service defined in hello.proto
#[tonic::async_trait]
impl Greeter for MyGreeter {
    // Define what type of stream we will return for SayHello
    type SayHelloStream =
        Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send + 'static>>;

    // This runs whenever a client calls SayHello
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Incoming request with a name
    ) -> Result<Response<Self::SayHelloStream>, Status> {
        
        let name = request.into_inner().name; // Extract the name from request

        // Create a channel to send messages to the client
        let (tx, rx) = mpsc::channel(4);

        // Spawn a new async task to send messages over time
        tokio::spawn(async move {
            for i in 1..=5 {
                let msg = HelloReply {
                    message: format!("Hello {name}! Message #{i}"),
                };

                // If client disconnects, stop sending
                if tx.send(Ok(msg)).await.is_err() {
                    break;
                }

                // Wait 500ms before sending the next message
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        // Convert our receiver into a gRPC stream and send back to client
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?; // Server address (IPv6 localhost)
    let greeter = MyGreeter::default(); // Create service instance

    println!("Server listening on {}", addr);

    // Start gRPC server and add our Greeter service
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
