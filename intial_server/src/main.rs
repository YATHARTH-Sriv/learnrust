use std::net::SocketAddr;

use axum::{routing::get, Router};




#[tokio::main]
async fn main() {
   let app=Router::new().route("/", get(root_handler));

   
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .expect("PORT must be a number");

    // Define address to run our server on (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 2], 3000));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://{}", addr);

    // Start the server
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler()->&'static str{
    "Hello World"
}
