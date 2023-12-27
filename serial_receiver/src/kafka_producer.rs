use kafka::producer::{Producer, Record};

pub(crate) fn kafka_producer(data_to_send: &[u8]) {
    let hosts = vec!["localhost:9092".to_owned()];
    let mut producer = Producer::from_hosts(hosts).create().unwrap();
    producer
        .send(&Record::from_value("serial_communication", data_to_send))
        .unwrap();
    println!("kafka producer -> {:?}", data_to_send);
}
