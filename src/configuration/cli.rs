use clap::Parser;

const MAXIMUM_MESSAGE_BODY: usize = 1_048_576;
const MINIMUM_DELAY_SECONDS: i32 = 0;
const MAXIMUM_DELAY_SECONDS: i32 = 900;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// URL of the Amazon SQS queue to which a message is sent
    #[arg(short, long)]
    pub queue: String,
    /// AWS region for the SQS queue
    #[arg(short, long, default_value = "us-east-1")]
    pub region: String,
    /// Message to send. The minimum size is one character. The maximum size is 1 MiB or 1,048,576 bytes
    #[arg(short, long, default_value = "{default: data}")]
    pub message: String,
    /// Length of time, in seconds to delay a specific message.
    #[arg(short, long, default_value = "0")]
    pub delay: i32,
    /// FIFO queues, organizes messages into distinct groups.
    /// Standard queues, enables fair queues.
    #[arg(short, long, default_value = "sqs-enqueue")]
    pub id: String,
}

impl Args {
    pub fn parse_and_validate() -> Result<Self, String> {
        let args = Args::parse();

        if args.delay < MINIMUM_DELAY_SECONDS || args.delay > MAXIMUM_DELAY_SECONDS {
            return Err("Delay must be between 0 and 900 seconds".to_string());
        }

        if args.message.is_empty() || args.message.len() > MAXIMUM_MESSAGE_BODY {
            return Err("Message cannot be empty or exceed 1 MiB (1,048,576 bytes)".to_string());
        }
        Ok(args)
    }
}
