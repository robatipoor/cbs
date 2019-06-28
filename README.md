# cbs
[![Crates.io](https://img.shields.io/crates/v/cbs.svg?style=plastic)](http://crates.io/crates/cbs)
[![Build Status](https://travis-ci.org/robatipoor/cbs.svg?branch=master)](https://travis-ci.org/robatipoor/cbs)
[![Build status](https://ci.appveyor.com/api/projects/status/kr9iog6hyw3jfgqu/branch/master?svg=true)](https://ci.appveyor.com/project/robatipoor/cbs/branch/master)
[![License](https://img.shields.io/crates/l/cbs.svg)](https://crates.io/crates/cbs/)
### Work in progress
cbs is a command line utility that is designed to run on linux system , macOs and maybe windows. It provides an interface to manage clipboard from the command line

### Install cbs

```sh
# you need install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# and then install cbs
cargo install cbs
```

## How to use

- copy text to clipboard: `cbs -c "Text to be copied to clipboard"`
- paste copied text: `cbs -p`
- copy from stdin: `cat exmaple-file.txt | cbs`

## Usage

```sh
USAGE:
    cbs [FLAGS] [OPTIONS]

FLAGS:
    -C, --clear      Clear content clipboard
    -h, --help       Prints help information
    -p, --paste      Paste text content
    -V, --version    Prints version information

OPTIONS:
    -c, --copy <content>    Set a text content
```