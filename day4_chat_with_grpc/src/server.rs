pub mod pb {
    tonic::include_proto!("chat");
}

use pb::ChatMessage;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug)]
pub struct ChatServer {}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;
type ChatResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl pb::chat_service_server::ChatService for ChatServer {
    type ChatMessageStreamingStream = ResponseStream;
    async fn chat_message_streaming(
        &self,
        request: Request<Streaming<ChatMessage>>,
    ) -> ChatResult<Self::ChatMessageStreamingStream> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(item) => {
                        println!("Received message: {:?} from {:?}", item.message, item.from);
                        tx.send(Ok(ChatMessage {
                            message: format!("Server : {}", item.message),
                            from: "Server".to_string(),
                        }))
                        .await
                        .unwrap();
                    }
                    Err(status) => {
                        println!("Error receiving message: {:?}", status);
                        break;
                    }
                }
            }

            println!("Chat session ended...");
        });

        let out = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(out) as Self::ChatMessageStreamingStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = ChatServer {};
    println!("Starting gRPC chat server...");
    Server::builder()
        .add_service(pb::chat_service_server::ChatServiceServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}
