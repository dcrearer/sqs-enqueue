use sqs_enqueue::configuration::Args;
use sqs_enqueue::messaging::{create_client, send_message};

#[tokio::main]
async fn main() -> Result<(), aws_sdk_sqs::Error> {
    env_logger::init();

    let args = match Args::parse_and_validate() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let client = create_client(args.region.clone()).await;
    send_message(&client, args).await?;

    Ok(())
}
