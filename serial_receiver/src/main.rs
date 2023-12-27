mod kafka_producer;
mod read_write_serial_port;
mod serial_connection;
use crate::read_write_serial_port::receive_data;
use kafka::producer::AsBytes;
use serial_connection::{list_available_ports, open_serial_port};
use serialport;
use serialport::{SerialPort};
use std::io::{Read, Write};


fn add_end_bit(mut data: Vec<u8>) -> Vec<u8> {
    let eof = &"eof".as_bytes().to_vec();
    data.append(&mut eof.clone());
    println!("{:?}", data.last());
    data
}

fn main() {
    list_available_ports();
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 9600;
    let mut port = open_serial_port(port_name, baud_rate);
    receive_data(port.try_clone().expect("Unable to clone port"));
}
