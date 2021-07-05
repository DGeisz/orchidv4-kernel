use orchidv4_kernel::kernel_io::websocket_server::run_server;

#[tokio::main]
async fn main() {
    env_logger::init();

    run_server("127.0.0.1:7200").await;
}
