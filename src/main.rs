use user_service::{kafka_consumer::IngestConsumer, kafka_producer::create_kafka_producer};

const DEFAULT_BROKERS: &str = "localhost:29092";
const DEFAULT_CONSUMER_GROUP_ID: &str = "1";
const DEFAULT_LISTEN_TOPIC: &str = "from_router";

#[tokio::main]
async fn main() {
    // take brokers and top to listen to, from arg, or environment, or default
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let brokers = args.pop().unwrap_or(DEFAULT_BROKERS.to_string());
    let group_id = args.pop().unwrap_or(DEFAULT_CONSUMER_GROUP_ID.to_string());
    let listen_topics = args
        .pop()
        .map(|l_ts| {
            l_ts.split(",")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
        })
        .unwrap_or(vec![DEFAULT_LISTEN_TOPIC.to_string()]);

    let producer = create_kafka_producer(DEFAULT_BROKERS).unwrap();
    let ingest_consumer = IngestConsumer::new(brokers, group_id, listen_topics, producer)
        .expect("Failed to create ingest consumer");
    ingest_consumer.run().await;
}
