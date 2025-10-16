use crate::configuration::Args;
use aws_sdk_sqs::Client;
use aws_sdk_sqs::types::MessageAttributeValue;

pub async fn send_message(client: &Client, args: Args) -> Result<(), aws_sdk_sqs::Error> {
    let attr = MessageAttributeValue::builder()
        .string_value("high")
        .data_type("String")
        .build()?;

    client
        .send_message()
        .queue_url(args.queue)
        .message_body(args.message)
        .delay_seconds(args.delay)
        .message_group_id(args.id)
        .message_attributes("priority", attr)
        .send()
        .await?;

    Ok(())
}