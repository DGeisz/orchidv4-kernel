use crate::kernel_io::ws_io::ws_command_parser::ws_message_consumer::{
    mock_ws_message_consumer, MessageConsumptionResponse, WsMessageConsumer,
};
use crate::kernel_io::ws_io::ws_server::run_server;
use futures_util::{SinkExt, StreamExt};
use log::info;
use mockall::predicate::eq;
use std::sync::Arc;
use tokio::sync::Barrier;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

/// This is a basic test of the server functionality.
/// Spins up two websocket clients for communication
#[tokio::test]
#[ignore] // This is ignored just cause it takes forever to run
async fn multi_client_test() {
    env_logger::init();

    let addr = "127.0.0.1:7200";
    let ws_addr = "ws://localhost:7200";

    /*
    Initialize the messages to and from the server
    */
    let req1 = String::from("FirstReq");

    let req2 = String::from("SecondReq");

    let no_op_req = String::from("NO_OP");

    let kill_req = String::from("KillReq");

    let res1 = String::from("Res1");
    let res1_c1 = res1.clone();
    let res1_c2 = res1.clone();

    let res2 = String::from("Res2");
    let res2_c1 = res2.clone();
    let res2_c2 = res2.clone();

    let kill_res = String::from("ResKill");
    let kill_res_c1 = kill_res.clone();
    let kill_res_c2 = kill_res.clone();

    /*
    Create a tokio runtime for running the clients
    in their own thread
    */

    let task = tokio::spawn(async move {
        /*
        Create url out of addr
        */
        let url = url::Url::parse(ws_addr).unwrap();

        sleep(Duration::from_millis(10)).await;

        /*
        Create the first client, and split her up
        */
        info!("Creating first ws client");

        let (ws1, _) = connect_async(url.clone())
            .await
            .expect("First client failed to connect");

        let (mut ws1_write, mut ws1_read) = ws1.split();

        info!("Creating second ws client");

        /*
        Same thing for second client
        */
        let (ws2, _) = connect_async(url.clone())
            .await
            .expect("Second client failed to connect");

        let (mut ws2_write, mut ws2_read) = ws2.split();

        /*
        Create a barrier to synchronize the flow of the test
        */
        let barrier = Arc::new(Barrier::new(4));

        let bc1 = barrier.clone();
        let bc2 = barrier.clone();
        let bc3 = barrier.clone();
        let bc4 = barrier.clone();
        let bc5 = barrier.clone();

        /*
        Alright, let's handle reads and writes in their own processes.
        Reads first
        */
        let read_handle = tokio::spawn(async move {
            /*
            Spawn separate tasks for different clients
            */
            let c1 = tokio::spawn(async move {
                bc1.wait().await;

                if let Message::Text(msg1) = ws1_read.next().await.unwrap().unwrap() {
                    assert_eq!(msg1, res1_c1);
                } else {
                    panic!("Didn't receive text from client 1 first read");
                }

                info!("Made it past the first read assertion");

                bc1.wait().await;

                if let Message::Text(msg2) = ws1_read.next().await.unwrap().unwrap() {
                    assert_eq!(msg2, res2_c1);
                } else {
                    panic!("Didn't receive text from client 1 second read");
                }

                bc1.wait().await;

                /*
                Wait for 3 to connect and make sure it's not getting any dregs
                from past writes
                */
                bc1.wait().await;

                /*
                No op occurs
                */

                bc1.wait().await;

                if let Message::Text(msg3) = ws1_read.next().await.unwrap().unwrap() {
                    assert_eq!(msg3, kill_res_c1);
                } else {
                    panic!("Didn't receive text from client 1 kill read");
                }
            });

            let c2 = tokio::spawn(async move {
                bc2.wait().await;

                if let Message::Text(msg1) = ws2_read.next().await.unwrap().unwrap() {
                    assert_eq!(msg1, res1_c2);
                } else {
                    panic!("Didn't receive text from client 2 first read");
                }

                bc2.wait().await;

                if let Message::Text(msg2) = ws2_read.next().await.unwrap().unwrap() {
                    assert_eq!(msg2, res2_c2);
                } else {
                    panic!("Didn't receive text from client 2 second read");
                }

                bc2.wait().await;

                /*
                Wait for 3 to connect and make sure it's not getting any dregs
                from past writes
                */
                bc2.wait().await;

                /*
                No-op occurs
                */

                bc2.wait().await;

                if let Message::Text(msg3) = ws2_read.next().await.unwrap().unwrap() {
                    assert_eq!(msg3, kill_res_c2);
                } else {
                    panic!("Didn't receive text from client 2 kill read");
                }
            });

            let (c1j, c2j) = tokio::join!(c1, c2);

            c1j.unwrap();
            c2j.unwrap();
        });

        /*
        Now let's handle writes
        */
        let write_handle = tokio::spawn(async move {
            let c1 = tokio::spawn(async move {
                bc3.wait().await;

                ws1_write.send(Message::Text(req1)).await.unwrap();

                bc3.wait().await;

                /*
                Now the next message is sent
                */
                bc3.wait().await;

                /*
                Wait for 3 to connect and make sure it's not getting any dregs
                from past writes
                */
                bc3.wait().await;

                /*
                Wait for the no-op
                */

                bc3.wait().await;

                sleep(Duration::from_millis(10)).await;

                ws1_write.send(Message::Text(kill_req)).await.unwrap();
            });

            let c2 = tokio::spawn(async move {
                bc4.wait().await;

                /*
                Now the first message is sent
                */

                bc4.wait().await;

                ws2_write.send(Message::Text(req2)).await.unwrap();

                bc4.wait().await;

                /*
                Wait for 3 to connect and make sure it's not getting any dregs
                from past writes
                */
                bc4.wait().await;

                ws2_write.send(Message::Text(no_op_req)).await.unwrap();

                bc4.wait().await;

                /*
                Now the kill request is sent
                */
            });

            let (c1j, c2j) = tokio::join!(c1, c2);

            c1j.unwrap();
            c2j.unwrap();
        });

        let client3_handle = tokio::spawn(async move {
            bc5.wait().await;
            bc5.wait().await;
            bc5.wait().await;

            let (ws3, _) = connect_async(url)
                .await
                .expect("Client 3 failed to connect");

            let (_, mut ws3_read) = ws3.split();

            /*
            We're going to attempt to read from this socket, but if a simple timer finishes
            faster, then we assume there weren't any new messages for us
            */

            let next_msg = ws3_read.next();

            tokio::select! {
                next = next_msg => {
                    panic!("Third ws client got a message when one hadn't been sent");
                }
                _ = sleep(Duration::from_millis(100)) => {
                    info!("No new messages for third client");
                }
            }

            bc5.wait().await;
            bc5.wait().await;
        });

        let (read, write) = tokio::join!(read_handle, write_handle);

        read.unwrap();
        write.unwrap();
    });

    let (_, t) = tokio::join!(run_server(addr, create_ws_message_mock), task);

    t.unwrap()
}

pub fn create_ws_message_mock() -> Box<dyn WsMessageConsumer> {
    let mut mock = mock_ws_message_consumer();

    let req1 = String::from("FirstReq");
    let req2 = String::from("SecondReq");
    let no_op_req = String::from("NO_OP");
    let kill_req = String::from("KillReq");
    let res1 = String::from("Res1");
    let res2 = String::from("Res2");
    let kill_res = String::from("ResKill");

    mock.expect_consume_ws_message()
        .with(eq(req1))
        .return_const(MessageConsumptionResponse::Message(res1));

    mock.expect_consume_ws_message()
        .with(eq(req2))
        .return_const(MessageConsumptionResponse::Message(res2));

    mock.expect_consume_ws_message()
        .with(eq(no_op_req))
        .return_const(MessageConsumptionResponse::None);

    mock.expect_consume_ws_message()
        .with(eq(kill_req))
        .return_const(MessageConsumptionResponse::Terminate(kill_res));

    Box::new(mock)
}
