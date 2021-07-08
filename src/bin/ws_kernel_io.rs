use orchidv4_kernel::kernel_io::ws_io::run_ws_io;

#[tokio::main]
async fn main() {
    env_logger::init();

    run_ws_io("127.0.0.1:7200").await;
}
