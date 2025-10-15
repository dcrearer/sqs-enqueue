use aws_config::BehaviorVersion;
use aws_sdk_sqs::{Client, config::Region};
use sqs_enqueue::configuration::Args;
use std::collections::HashMap;
use aws_sdk_sqs::types::MessageAttributeValue;
use aws_sdk_sqs::types::{MessageSystemAttributeNameForSends, MessageSystemAttributeValue};



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

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(args.region))
        .load()
        .await;

    let attr = MessageAttributeValue::builder()
        .string_value("high")
        .data_type("String")
        .build()?;

    let mut system_attrs = HashMap::new();
    system_attrs.insert(
        MessageSystemAttributeNameForSends::AwsTraceHeader,
        MessageSystemAttributeValue::builder()
            .string_value("Root=1-5e1b4151-5ac6c58f4c84240da44aa5d8")
            .data_type("String")
            .build()?
    );

    let client = Client::new(&config);
    let _ = client
        .send_message()
        .queue_url(args.queue)
        .message_body(args.message)
        .delay_seconds(args.delay)
        .message_group_id(args.id)
        .message_attributes("priority", attr)
        .set_message_system_attributes(Some(system_attrs))
        .send()
        .await?;

    Ok(())
}
