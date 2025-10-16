use crate::configuration::Args;
use aws_sdk_sqs::Client;
use aws_sdk_sqs::types::{SendMessageBatchRequestEntry, MessageAttributeValue};

pub async fn send_batch(client: &Client, messages: Vec<Args>) -> Result<(), aws_sdk_sqs::Error> {
    if messages.is_empty() {
        return Ok(());
    }

    let queue_url = messages[0].queue.clone();
    
    let mut entries = Vec::new();
    
    for (i, args) in messages.into_iter().enumerate() {
        let attr = MessageAttributeValue::builder()
            .string_value("high")
            .data_type("String")
            .build()?;

        let entry = SendMessageBatchRequestEntry::builder()
            .id(format!("msg-{}", i))
            .message_body(args.message)
            .delay_seconds(args.delay)
            .message_group_id(args.id)
            .message_attributes("priority", attr)
            .build()?;
            
        entries.push(entry);
    }

    client
        .send_message_batch()
        .queue_url(queue_url)
        .set_entries(Some(entries))
        .send()
        .await?;

    Ok(())
}