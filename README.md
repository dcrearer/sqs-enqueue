# sqs-enqueue

A simple CLI tool to send messages to Amazon SQS queues.

## Usage

Usage: sqs-enqueue [OPTIONS] --queue <QUEUE>

Options:
-q, --queue <QUEUE>      URL of the Amazon SQS queue to which a message is sent
-r, --region <REGION>    AWS region for the SQS queue [default: us-east-1]
-m, --message <MESSAGE>  Message to send. The minimum size is one character. The maximum size is 1 MiB or 1,048,576 bytes [default: "{default: data}"]
-d, --delay <DELAY>      Length of time, in seconds to delay a specific message [default: 0]
-i, --id <ID>            FIFO queues, organizes messages into distinct groups. Standard queues, enables fair queues [default: sqs-enqueue]
-h, --help               Print help
-V, --version            Print version


## Examples

```bash
# Send default message
sqs-enqueue --queue https://sqs.us-east-1.amazonaws.com/123456789012/my-queue 

# Send with delay and custom region
sqs-enqueue -r us-west-2 -d 30 https://sqs.us-west-2.amazonaws.com/123456789012/my-queue "Delayed message"
```

## Requirements

- AWS credentials configured (via AWS CLI, environment variables, or IAM roles)
- Rust 1.90.0 used to build

## Installation

```bash
cargo build --release
./target/release/sqs-enqueue --help
```
