use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use std::net::SocketAddr;
use std::thread;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio::sync::watch::Receiver;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn run_server(addr: &str) {
    let (out_tx, mut out_rx) = watch::channel(0);
    let (in_tx, mut in_rx) = mpsc::channel(32);

    out_tx.send(0);

    thread::spawn(move || loop {
        let mut counter = 0;

        while let Some(incr) = in_rx.blocking_recv() {
            info!("Received message: {}", incr);

            if incr {
                counter += 1;
            } else {
                counter -= 1;
            }

            out_tx.send(counter);
        }
    });

    let listener = TcpListener::bind(addr)
        .await
        .expect("Server was unable to connect");

    info!("Listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => match stream.peer_addr() {
                Ok(peer) => {
                    let in_tx_clone = in_tx.clone();
                    let out_rx_clone = out_rx.clone();

                    tokio::spawn(handle_connection(stream, peer, in_tx_clone, out_rx_clone));
                }
                Err(e) => error!("Unable to acquire socket peer address: {}", e),
            },
            Err(e) => error!("Failed to connect {}", e),
        }
    }
}

pub async fn handle_connection(
    stream: TcpStream,
    peer: SocketAddr,
    in_tx: Sender<bool>,
    mut out_rx: Receiver<i32>,
) {
    info!("New websocket connection: {}", peer);

    match accept_async(stream).await {
        Ok(mut ws_stream) => {
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();
            let (one_tx, mut one_rx) = oneshot::channel();

            let peer_clone = peer.clone();

            tokio::spawn(async move {
                while out_rx.changed().await.is_ok() {
                    info!(
                        "Getting something from out_rx: {}",
                        (*out_rx.borrow()).to_string()
                    );

                    let next = (*out_rx.borrow()).to_string();

                    ws_sender.send(Message::Text(next)).await;

                    if let Ok(_) = one_rx.try_recv() {
                        info!("Dropping ws stream from: {}", peer_clone);
                        break;
                    }
                }
            });

            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(msg) => {
                        if msg.is_text() {
                            info!("The message is text: {:?}", msg);

                            in_tx.send(true).await;
                        }
                    }
                    Err(e) => error!("Error parsing ws stream {} message: {}", peer, e),
                }
            }

            one_tx.send(true);
            in_tx.send(false).await;

            info!("Dropping connection: {}", peer);
        }
        Err(e) => error!("Error processing connection for {}: {}", peer, e),
    }
}
