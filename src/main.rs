use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //Bubbleup the io::Errorifwe failed tobindtheaddress
    //Otherwise call.await on our Server

    run(TcpListener::bind("127.0.0.1:8000").expect("BIND FAILED"))?.await
}
