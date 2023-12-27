use kafka::client::FetchOffset;
use kafka::consumer::Consumer;

fn main() {
    let host = vec!["localhost:9092".to_owned()];

    let mut consumer = Consumer::from_hosts(host)
        .with_topic("serial_communication".to_owned())
        .with_fallback_offset(FetchOffset::Latest)
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("{:?}",std::str ::from_utf8(m.value));
            }
            consumer.consume_messageset(ms).unwrap();
        }
        consumer.commit_consumed().unwrap();
    }
}
