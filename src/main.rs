use log::Level;
use tokio::{io::AsyncReadExt, time};

mod tcpudp;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    // define runtime
    // run().await;
    tcpudp::run().await;
}

async fn run() {
    tokio::join!(level1(), level2());
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

async fn level1() {
    log::info!("Sleeping");
    time::sleep(time::Duration::from_secs(2)).await;
    log::info!("Awake");
}

async fn level2() {
    log::info!("Reading some data");
    let mut f = tokio::fs::File::open("test.txt").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read some data: {} bytes", contents.len());

    tokio::task::spawn_blocking(move || {
        log::info!("Computing fib(40)");
        fib(40);
        log::info!("Finished computing fib(40)");
    })
    .await
    .unwrap();
}
