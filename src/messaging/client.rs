use aws_config::BehaviorVersion;
use aws_sdk_sqs::{Client, config::Region};

pub async fn create_client(region: String) -> Client {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region))
        .load()
        .await;

    Client::new(&config)
}
