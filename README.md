# sqs-enqueue

A simple CLI tool to send messages to Amazon SQS queues.

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
