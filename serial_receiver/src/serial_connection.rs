use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::time::Duration;

pub(crate) fn list_available_ports() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}

pub(crate) fn open_serial_port(port_name: &str, baud_rate: u32) -> Box<dyn SerialPort> {
    serialport::new(port_name, baud_rate)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(10000))
        .open()
        .expect("Failed to open port")
}
