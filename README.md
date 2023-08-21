# Fluere

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FSkuldNorniern%2Ffluere.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FSkuldNorniern%2Ffluere?ref=badge_shield)
[![Rust](https://github.com/SkuldNorniern/fluere/actions/workflows/rust.yml/badge.svg)](https://github.com/SkuldNorniern/fluere/actions/workflows/rust.yml)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/9bb831ce9bab4ed394763bf9d6583773)](https://www.codacy.com/gh/SkuldNorniern/fluere/dashboard?utm_source=github.com&utm_medium=referral&utm_content=SkuldNorniern/fluere&utm_campaign=Badge_Grade)
[![Drone Build Status](https://drone.nornity.com/api/badges/SkuldNorniern/fluere/status.svg)](https://drone.nornity.com/SkuldNorniern/fluere)
## Cross Platform Packet Capture, pcap to Netflow Conversion, Live Netflow Capture Tool

<p align="center" align="right">
  Supported Platforms
</p>
<p align="center" align="right">
  <img alt="Windows" src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white"/>
  <img alt="MacOS" src="https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=macos&logoColor=F0F0F0"/>
  <img alt="Linux" src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black"/>
</p>
<p align="center" align="right">
  Windows, MacOS, and Linux are All Supported! YAY!
</p>

<div align="center">
    <img src="https://github.com/SkuldNorniern/fluere/blob/main/images/main.png" alt="Help Image" width="770" height="401"></img>
</div>

## Project Description

Fluere is a powerful and versatile tool designed for network monitoring and analysis. It is capable of capturing network packets in pcap format and converting them into NetFlow data, providing a comprehensive view of network traffic. Fluere supports both live capture and offline conversion of NetFlow data, making it suitable for a wide range of use cases. Additionally, Fluere offers a terminal user interface for live feedback during online capture. Fluere is cross-platform compatible, running on Windows, macOS, and Linux operating systems.

The project is implemented in Rust and uses the `libpcap` library for packet capture and the `clap` library for command line argument parsing. The main functionality of the project is defined in the `main.rs` file, which includes the definition of the command line interface and the handling of the different commands and options.

## Arguments

The following table provides detailed information about each argument:

| Argument | Purpose | Usage |
| --- | --- | --- |
| csv | Title of the exported csv file | `-c` or `--csv` |
| list | List of network interfaces | `-l` or `--list` |
| interface | Select network interface to use | `-i` or `--interface` |
| duration | Set capture duration, in milliseconds | `-d` or `--duration` |
| timeout | Set flow timeout, in milliseconds | `-t` or `--timeout` |
| useMACaddress | Set use MAC address on Key value | `-M` or `--useMAC` |
| interval | Set export interval, in milliseconds | `-I` or `--interval` |
| sleep_windows | Set interval of thread pause for (only)MS Windows per n packet | `-s` or `--sleep` |
| verbose | Set verbosity level | `-v` or `--verbose` |

## Usage

To use Fluere, you first need to install it using the `cargo install fluere` command. Once installed, you can run Fluere using the `fluere` command followed by the desired subcommand (`online`, `offline`, `pcap`) and the corresponding options.

### Examples of Common Use Cases

1. **Live NetFlow Capture and Conversion**: To capture NetFlow data in real-time from a specific network interface, use the `online` subcommand. For example:

```sh
fluere online -i eth0 -d 1000 -t 600000 -I 1800000 -v 1
```

This command captures NetFlow data from the `eth0` interface for a duration of 1000 milliseconds, with a flow timeout of 600000 milliseconds and an export interval of 1800000 milliseconds. The verbosity level is set to 1.

2. **Offline pcap to NetFlow Conversion**: To convert a pcap file into NetFlow data, use the `offline` subcommand. For example:

```sh
fluere offline -f input.pcap -c output
```

This command converts the `input.pcap` file into NetFlow data and exports the data to a CSV file named `output.csv`.

3. **Packet Capture in pcap Format**: To capture packets in pcap format from a specific network interface, use the `pcap` subcommand. For example:

```sh
fluere pcap -i eth0 -d 1000
```

This command captures packets from the `eth0` interface for a duration of 1000 milliseconds and saves the packets in a pcap file.

For example, to capture netflow online and export it to a csv file, you can use the following command:

```sh
fluere online -c output -i eth0 -d 1000 -t 600000 -I 1800000 -v 1
```

This command will capture netflow data from the `eth0` interface for a duration of 1000 milliseconds, with a flow timeout of 600000 milliseconds and an export interval of 1800000 milliseconds. The verbosity level is set to 1 and the output will be saved to a csv file named `output.csv`.

## Prerequisites

Ensure that you have installed `libpcap` on Linux and macOS or `npcap` on Windows.
- you need to install `npcap` in `winpcap compatible mode` 

## Installation

```sh
cargo install fluere
```

## Usage

Execute Fluere by entering the `fluere` command in the terminal.

To list available interfaces, use:

```sh
fluere online -l
```

or

```sh
fluere pcap -l
```

Select the capture mode:

- `online`: Live NetFlow data capture and conversion
- `offline`: Convert pcap files to NetFlow data
- `pcap`: Capture packets in pcap format

Specify the desired capture duration in milliseconds (ms):

```sh
-d 1000
```

Set the output file's title:

```sh
-c file_title
```

The captured packets or NetFlow data will be saved in the "output" directory within Fluere's installation folder.

## Important Notes

For Linux and macOS users, ensure that you run Fluere with administrator privileges.

### Example

```sh
sudo fluere online -d 1000 -c my_capture
```

## License

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FSkuldNorniern%2Ffluere.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FSkuldNorniern%2Ffluere?ref=badge_large)
