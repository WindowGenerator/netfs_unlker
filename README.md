# netfs-unlker

netfs-unlker is a Rust library and command-line interface designed to provide unlock 'fcntl' shared locks in netapp's

## Features

- **Library**: Core functionalities that can be integrated into other Rust applications.
- **Command-Line Interface**: For users who prefer direct command line access, `netfs-unlker` is available when built with the appropriate features.

## Getting Started

### Prerequisites

Make sure you have Rust installed on your machine. If not, you can install Rust using [rustup](https://rustup.rs/).

### Installation

Clone the repository:

```bash
git clone https://github.com/WindowGenerator/netfs_unlker.git
cd netfs_unlker
```

To build the command-line interface, you need to enable the corresponding feature:

```bash
cargo build --features build-netfs-unlker-cli
```
Usage
As a Library
You can include netfs-unlker in your Rust projects by adding it to your Cargo.toml:

```toml
[dependencies]
netfs_unlker = { version = "0.1.0", path = "path_to_netfs_unlker" }
```
Then, use it in your Rust application:

```rust
extern crate netfs_unlker;
```

Example usage
Command Line Interface
If the CLI has been built, you can run it using:

```bash
./target/debug/netfs-unlker [OPTIONS]
```

Replace [OPTIONS] with the command line options you provide. (Expand this section based on the actual functionality of your CLI.)

### Contributing
Contributions are welcome! Please feel free to submit pull requests or create issues for bugs and feature requests.

### License
This project is licensed under the MIT License - see the LICENSE file for details.

### More Information
For more details, you can refer to the official Rust documentation: Cargo Manifest Format.