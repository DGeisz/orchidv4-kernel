//! The actual tokio websocket server that receives and
//! handles open websocket connections.  This module's sole
//! purpose is to maintain websockets, pipe text received
//! from websockets to a consumption port, and send
//! any output from the consumption port back through open
//! websocket connections.

use crate::kernel_io::ws_io::ws_message_consumption_port::{
    MessageConsumptionResponse, WsMessageConsumptionPort,
};
use futures_util::{SinkExt, StreamExt};
use log::{error, info, trace};
use std::net::SocketAddr;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, mpsc::Sender, oneshot, watch, watch::Receiver},
};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[cfg(test)]
mod tests;

pub fn run_server(addr: &'static str, consumption_port: &impl WsMessageConsumptionPort) {
    /*
    First initialize the channels used for
    communication between tasks
    */
    let (out_tx, out_rx) = watch::channel(String::new());
    let (in_tx, mut in_rx) = mpsc::channel::<String>(32);

    let _unused_clone = in_tx.clone();

    /*
    Create a multi-threaded runtime and spawn
    a background task to handle all ws connections
    */
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .spawn(async move {
            let listener = TcpListener::bind(addr)
                .await
                .expect("Server was unable to connect");

            info!("Listening on {}", addr);

            /*
            Loop through all new connections, handling each one
            */
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        info!("Got a new stream!");

                        match stream.peer_addr() {
                            Ok(peer) => {
                                /*
                                At this point we have all the info necessary to handle
                                the new connection, so clone the necessary channels and
                                spawn a new task to handle the connection
                                */
                                let in_tx_clone = in_tx.clone();
                                let out_rx_clone = out_rx.clone();

                                tokio::spawn(handle_connection(
                                    stream,
                                    peer,
                                    in_tx_clone,
                                    out_rx_clone,
                                ));
                            }
                            Err(e) => error!("Unable to acquire socket peer address: {}", e),
                        }
                    }
                    Err(e) => error!("Failed to accept new connection: {}", e),
                }
            }
        });

    /*
    In the main thread, loop on blocking receives to consume
    the next incoming ws messages.  Blocking_recv only returns
    None once the channel is closed, hence the while let.
    */
    while let Some(msg) = in_rx.blocking_recv() {
        trace!("Received message: {}", msg);

        /*
        Consume the message, and if the message produces an output
        send the message out to all open ws connections watching out_rx
        */
        match consumption_port.consume_ws_message(msg) {
            MessageConsumptionResponse::Message(output) => {
                /*
                We have some text we should send back to the clients
                */
                if let Err(e) = out_tx.send(output) {
                    error!("Error sending consumption output to ws connections: {}", e);
                }
            }
            MessageConsumptionResponse::Terminate(output) => {
                /*
                Time to terminate the server, but let's send the
                final message first
                */
                if let Err(e) = out_tx.send(output) {
                    error!(
                        "Error sending consumption termination output to ws connections: {}",
                        e
                    );
                }

                break;
            }
            MessageConsumptionResponse::None => (),
        }
    }

    info!("Shutting down the server");
}

pub async fn handle_connection(
    stream: TcpStream,
    peer: SocketAddr,
    in_tx: Sender<String>,
    mut out_rx: Receiver<String>,
) {
    info!("New ws connection: {}", peer);

    /*
    First attempt to accept the new connection
    */
    match accept_async(stream).await {
        Ok(ws_stream) => {
            /*
            Ok, now that we have the actual ws stream, split it into
            a sender and receiver
             */
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();

            /*
            We'll create a special task for the ws sender, so create a oneshot channel
            for communicating between the sender and receiver
            */
            let (one_tx, mut one_rx) = oneshot::channel();

            let peer_clone = peer.clone();

            /*
            Spawn a new process specifically for receiving the output of
            consumed messages and beaming it back to through the ws connection
            */
            tokio::spawn(async move {
                /*
                Loop over changes to out_rx which is the channel we're
                watching for message consumer output
                */
                while out_rx.changed().await.is_ok() {
                    let next_output = (out_rx.borrow()).to_string();

                    trace!(
                        "Ws {} received new output from out_rx: {}",
                        peer_clone,
                        next_output
                    );

                    /*
                    Send the message back through the ws connection
                    */
                    if let Err(e) = ws_sender.send(Message::Text(next_output)).await {
                        error!(
                            "Error sending output through ws connection {}: {}",
                            peer_clone, e
                        )
                    }

                    /*
                    Now we check if the oneshot channel has been activated, because
                    that indicates the ws connection has been dropped and we can end this process
                    */
                    if let Ok(_) = one_rx.try_recv() {
                        info!("Dropping ws stream: {}", peer_clone);
                        break;
                    }
                }
            });

            /*
            Now in the main part of the task we handle receiving
            new ws messages. Basically just loop over the incoming stream
            */
            while let Some(msg) = ws_receiver.next().await {
                /*
                Check to be sure our message is actually a message
                */
                match msg {
                    Ok(msg) => {
                        /*
                        Let's be sure the message is text
                        */
                        if let Message::Text(msg_content) = msg {
                            trace!(
                                "Received message {} from ws connection on {}",
                                msg_content,
                                peer
                            );

                            /*
                            Finally, send the message in to be consumed. Retry 3 times
                            in case of errors
                            */
                            for i in 0..3 {
                                if let Err(e) = in_tx.send(msg_content.clone()).await {
                                    error!(
                                        "Error sending ws msg content to \
                                        message consumer from {} on try {}: {}",
                                        peer,
                                        i + 1,
                                        e
                                    )
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => error!("Error parsing ws stream {} message: {}", peer, e),
                }
            }

            /*
            Once the stream has been closed (which happens in ws_rec.next returns none)
            we want to send a message through the oneshot to indicate that the sender
            process should stop looping
            */
            if let Err(e) = one_tx.send(true) {
                error!("Error sending oneshot message for {}: {}", peer, e)
            }

            info!("Dropping ws connection: {}", peer);
        }
        Err(e) => error!("Error processing ws stream {}: {}", peer, e),
    }
}
