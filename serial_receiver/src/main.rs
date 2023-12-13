use serialport;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

fn list_available_ports() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}

fn open_serial_port(port_name: &str, baud_rate: u32) -> Box<dyn SerialPort> {
    serialport::new(port_name, baud_rate)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(10000))
        .open()
        .expect("Failed to open port")
}

fn receive_data(mut port: Box<dyn SerialPort>) {
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
                        println!("Received: {:?}", utf_8_data);

                        // if utf_8_data == "\n" {
                        //
                        //     std::thread::sleep(Duration::from_millis(10000));
                        //     send_acknowledgement(port.try_clone().expect("Unable to clone port"));
                        //     break;
                        // }
                        port.clear(serialport::ClearBuffer::Input)
                            .expect("Unable to clear buffer");
                        std::thread::sleep(Duration::from_millis(50));
                    } else {
                        println!("Received non-UTF-8 data: {:?}", &received_data);
                    }
                } else   {
                    println!("Bytes read is 0. Calling handle_zero_bytes_read()...");

                    std::thread::sleep(Duration::from_millis(10000));
                    send_acknowledgement(port.try_clone().expect("Unable to clone port"));
                    break;
                }
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                std::thread::sleep(Duration::from_millis(10000));
                send_acknowledgement(port.try_clone().expect("Unable to clone port"));
                break;
               // eprintln!("Timeout occurred. Waiting for more data...");
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
                break;
            }
        }
    }
}

fn send_acknowledgement(mut port: Box<dyn SerialPort>) {
    let mut file =
        File::open("resources/894.txt").expect("Unable to open 894 acknowledgement file");
    let mut file_buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_buffer).expect("Unable to write");
    let extra_buffer = file_buffer.clone();
    let size_of_buffer = extra_buffer.len();

    std::thread::sleep(Duration::from_millis(50));

    for mut data in extra_buffer.chunks(size_of_buffer / 1) {
        port.clear(serialport::ClearBuffer::Output)
            .expect("Unable to clear buffer");
        port.write(&mut data).expect("Write failed");
        port.flush().expect("Unable to flush");
        std::thread::sleep(Duration::from_millis(500));
        println!("Acknowledgment sent -> {:?}", data);
    }
}

fn main() {
    list_available_ports();
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 115_200;
    let mut port = open_serial_port(port_name, baud_rate);
    receive_data(port.try_clone().expect("Unable to clone port"));
}
