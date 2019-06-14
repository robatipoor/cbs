# cbs
[![Crates.io](https://img.shields.io/crates/v/cbs.svg?style=plastic)](http://crates.io/crates/cbs)
[![Build Status](https://travis-ci.org/robatipoor/clipboard-server.svg?branch=master)](https://travis-ci.org/robatipoor/clipboard-server)
[![License](https://img.shields.io/crates/l/cbs.svg)](https://crates.io/crates/cbs/)
### work in progress
Command line tool to manage clipboard

**install**

```sh
cargo install cbs
```

**Build Manually**

```sh
# build and install cbs 
# need git, rustc, cargo, gnu make, binutils, upx
git clone https://github.com/robatipoor/cbs \
&& cd cbs \
&& make 
```

## How to use

- Copy text: `cbs -c "Text to be copied to clipboard"`
- Paste copied text: `cbs -p`
- Copy from stdin: `cat file | cbs`

## Usage

```sh
USAGE:
    cbs [FLAGS] [OPTIONS]

FLAGS:
    -C, --clear      Clear content clipboard
    -h, --help       Prints help information
    -k, --kill       Kill clipboard daemon server
    -l, --log        Show logs
    -p, --paste      Paste text content
    -s, --server     Start server clipboard as daemon
    -V, --version    Prints version information

OPTIONS:
    -c, --copy <content>    Set a text content
```