use bookmark_adapter::bookmark_shared_adapters::email_templates::EmailBuilder;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

pub mod handlers;
pub mod config;

fn main() {
    let mut consumer = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_topic_partitions("bookmark".to_owned(), &[0, 1])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("bookmark_apps".to_owned())
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();
    loop {
        for payload in consumer.poll().unwrap().iter() {
            for data in payload.messages() {
                println!("{:#?}", bincode::deserialize::<EmailBuilder>(data.value));
            }
            let _ = consumer.consume_messageset(payload);
        }
        consumer.commit_consumed().unwrap();
    }
}

// kafka-topics --bootstrap-server localhost:9092 --topic bookmark --create --partitions 3 --replication-factor 1
