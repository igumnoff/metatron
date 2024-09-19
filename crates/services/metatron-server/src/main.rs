use metatron_server::router;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(
    name = "metatron",
    author,
    version,
    about = "Metatron: Implementation in Rust of a report generation based on Shiva library",
    long_about = None
)]
struct Args {
    #[arg(
        short,
        long, 
        default_value_t = 3000, 
        help = "The port number to bind the server to",
    )]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let address = format!("0.0.0.0:{}", args.port);
    println!("Listening on {}", address);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, router()).await.unwrap();
}
