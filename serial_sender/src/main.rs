
/*

use serialport::SerialPort;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

fn main() {
    
    let mut file = File::open("resources/857.txt").expect("unable to open file");
    let mut file_buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_buffer).expect("unable to write");
    let mut extra_buffer = file_buffer.clone();

    let size_of_buffer = file_buffer.len();

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/ttyS0", 9600)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::One)
        .parity(Parity::None)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(10000))
        .open()
        .expect("Failed to open port");

    println!("size of buffer{}", size_of_buffer);

    for mut data in extra_buffer.chunks(size_of_buffer / 8) {
        std::thread::sleep(Duration::from_millis(50));
        
        port.clear(serialport::ClearBuffer::Output)
            .expect("unable to clear buffer");
            
        port.write(&mut data).expect("write failed");
        port.flush().expect("unable to clear buffer");
        std::thread::sleep(Duration::from_millis(500));

        println!("write data -> {:?}", data);

        if data == &[10] {
            //std::thread::sleep(Duration::from_millis(500));
            println!("data{:?}", data);
            receive_acknowledgement(port.try_clone().expect("unable to clone "));
            break;
        }
    }
}

//function to read acknowledgement
fn receive_acknowledgement(mut port: Box<dyn SerialPort>) {
    
    let mut buffer: Vec<u8> = vec![0; 1024];
    let mut output_file = File::create("resources/acknowledgement.txt")
        .expect("unable to create acknowledgement file");
port.clear(serialport::ClearBuffer::Input)
            .expect("unable to clear buffer");
    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let received_data = &buffer[..bytes_read];
                    if std::str::from_utf8(received_data).is_ok() {
                        let utf_8_data = String::from_utf8_lossy(received_data);

                        output_file
                            .write_all(&received_data)
                            .expect("unable to write data in file");

                        println!("acknowledgement -> {:?}", utf_8_data);

                        std::thread::sleep(Duration::from_millis(50));
                    } else {
                        println!("Received non-utf data {:?}", &received_data);
                    }
                }
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                eprintln!("time out occured . Waiting for more data ....");
            }
            Err(e) => {
                eprintln!("Error reading from serial port : {}", e);
                break;
            }
        }
    }
}

*/





use serialport::SerialPort;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn open_file(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Unable to read file");
    buffer
}

fn list_available_ports() {
    let ports = serialport::available_ports().expect("No ports found!");

    for p in ports {
        println!("{}", p.port_name);
    }
}

fn initialize_serial_port(port_name: &str, baud_rate: u32) -> Box<dyn SerialPort> {
    serialport::new(port_name, baud_rate)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(10000))
        .open()
        .expect("Failed to open port")
}

fn write_to_serial_port(port: &mut Box<dyn SerialPort>, data: &[u8]) {
    thread::sleep(Duration::from_millis(50));

    port.clear(serialport::ClearBuffer::Output)
        .expect("Unable to clear buffer");

    port.write(data).expect("Write failed");
    port.flush().expect("Unable to flush buffer");
    thread::sleep(Duration::from_millis(500));

    println!("Write data -> {:?}", data);
}

fn receive_acknowledgement(mut port: Box<dyn SerialPort>) {
    
    let mut buffer: Vec<u8> = vec![0; 1024];
    
    let mut output_file = File::create("resources/acknowledgement.txt")
        .expect("Unable to create acknowledgement file");
                        //thread::sleep(Duration::from_millis(50));
    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let received_data = &buffer[..bytes_read];
                    if let Ok(utf_8_data) = std::str::from_utf8(received_data) {
                        output_file
                            .write_all(received_data)
                            .expect("Unable to write data in file");

                        println!("Acknowledgement -> {:?}", utf_8_data);

                        if(utf_8_data.contains("/n")) {
                            break;
                            }


                       // thread::sleep(Duration::from_millis(50));
                    } else {
                        println!("Received non-UTF-8 data {:?}", &received_data);
                    }
                }
                
                
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                eprintln!("Timeout occurred. Waiting for more data....");
            }

            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
                break;
            }
        }
    }
}



fn main() {
    let file_buffer = open_file("resources/857.txt");
    let size_of_buffer = file_buffer.len();

    list_available_ports();

    let mut port = initialize_serial_port("/dev/ttyUSB0", 9600);

    for data in file_buffer.chunks(size_of_buffer / 8) {
        write_to_serial_port(&mut port, data);
        
        if data == &[10] {
            println!("Data: {:?}", data);
          //  thread::sleep(Duration::from_millis(50));

            receive_acknowledgement(port.try_clone().expect("Unable to clone port"));
            break;
        }
    }
}

