use crate::add_end_bit;
use crate::kafka_producer::kafka_producer;
use log::info;
use serialport::SerialPort;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

fn send_acknowledgement(mut port: Box<dyn SerialPort>) {
    let mut file =
        File::open("resources/894.txt").expect("Unable to open 894 acknowledgement file");
    let mut file_buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_buffer).expect("Unable to write");
    let extra_buffer = file_buffer.clone();
    let size_of_buffer = extra_buffer.len();
    let after_eof = add_end_bit(file_buffer);
    std::thread::sleep(Duration::from_millis(50));
    for mut data in after_eof.chunks(size_of_buffer / 1) {
        port.clear(serialport::ClearBuffer::Output)
            .expect("Unable to clear buffer");
        port.write(&mut data).expect("Write failed");
        port.flush().expect("Unable to flush");
        std::thread::sleep(Duration::from_millis(500));
        info!("Acknowledgment sent -> {:?}", std::str::from_utf8(data));
    }
}

pub(crate) fn receive_data(mut port: Box<dyn SerialPort>) {
    let mut buffer: Vec<u8> = vec![0; 2028];
    let mut output_file = File::create("resources/received.txt").expect("Unable to create a file");
    port.clear(serialport::ClearBuffer::All)
        .expect("Unable to clear buffer");
    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let received_data = &buffer[..bytes_read];
                    if let Ok(utf_8_data) = std::str::from_utf8(received_data) {
                        output_file
                            .write_all(received_data)
                            .expect("Unable to write data");
                        kafka_producer(received_data);
                        println!("Received: {:?}", utf_8_data);

                        if utf_8_data.contains("eof") {
                            info!("inside if ");
                            std::thread::sleep(Duration::from_millis(10000));
                            send_acknowledgement(port.try_clone().expect("Unable to clone port"));
                            break;
                        }
                        port.clear(serialport::ClearBuffer::Input)
                            .expect("Unable to clear buffer");
                        std::thread::sleep(Duration::from_millis(50));
                    } else {
                        info!("Received non-UTF-8 data: {:?}", &received_data);
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                eprintln!("Timeout occurred. Waiting for more data...");
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
                break;
            }
        }
    }
}
