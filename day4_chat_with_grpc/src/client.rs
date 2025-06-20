pub mod pb {
    tonic::include_proto!("chat");
}

use pb::{chat_service_client::ChatServiceClient, ChatMessage};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use tonic::transport::Channel;
use tonic::Request;

pub async fn input() -> String{
    println!("Enter your message: ");
    let mut inp = String::new();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    reader.read_line(&mut inp).await.expect("Failed to read line");
    inp.trim().to_string()
}

async fn chat(client : &mut ChatServiceClient<Channel>){
    let (tx, rx) = mpsc::channel(128);
    let mut in_stream = ReceiverStream::new(rx);

    tokio::spawn(async move {
        loop {
            let user_msg = input().await;
            if user_msg.eq_ignore_ascii_case("exit") {
                println!("Exiting chat...");
                break;
            } else {
                let msg = ChatMessage {
                    message: user_msg,
                    from: "Client".to_string(),
                };

                if tx.send(msg).await.is_err() {
                    println!("Failed to send message");
                    break;
                }
            }
        }
    });

    let resp = client
        .chat_message_streaming(Request::new(in_stream))
        .await
        .expect("Failed to start chat");

    let mut resp_stream = resp.into_inner();

    while let Some(result) = resp_stream.next().await {
        match result {
            Ok(msg) => {
                println!("Received message: {} from {}", msg.message, msg.from);
            }
            Err(status) => {
                eprintln!("Error receiving message: {:?}", status);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatServiceClient::connect("http://[::1]:50051").await.unwrap();
    println!("Connected to chat server...");

    chat(&mut client).await;

    Ok(())
}

