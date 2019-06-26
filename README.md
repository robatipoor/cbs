# cbs
[![Crates.io](https://img.shields.io/crates/v/cbs.svg?style=plastic)](http://crates.io/crates/cbs)
[![Build Status](https://travis-ci.org/robatipoor/cbs.svg?branch=master)](https://travis-ci.org/robatipoor/cbs)
[![Build status](https://ci.appveyor.com/api/projects/status/kr9iog6hyw3jfgqu/branch/master?svg=true)](https://ci.appveyor.com/project/robatipoor/cbs/branch/master)
[![License](https://img.shields.io/crates/l/cbs.svg)](https://crates.io/crates/cbs/)
### work in progress
cbs is a command line utility that is designed to run on linux system , macos and maybe windows. It provides an interface to manage clipboard from the command line

**install**

```sh
cargo install cbs
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
    -p, --paste      Paste text content
    -V, --version    Prints version information

OPTIONS:
    -c, --copy <content>    Set a text content
```