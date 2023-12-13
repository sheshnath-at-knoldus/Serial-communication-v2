
# serial-communication -v2

we can establish a connection between two devices via serial port . we can use serial ports to transfer data using RS232 . 

## Dependencies
Rust versions 1.59.0 and higher are supported by the library itself. There are examples requiring newer versions of Rust.

For GNU/Linux pkg-config headers are required:

- Ubuntu: sudo apt install pkg-config
- Fedora: sudo dnf install pkgconf-pkg-config
For other distros they may provide pkg-config through the pkgconf package instead.

For GNU/Linux libudev headers are required as well (unless you disable the default libudev feature):

- Ubuntu: sudo apt install libudev-dev
- Fedora: sudo dnf install systemd-devel
