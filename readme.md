# ACF Field Generator CLI

## Overview

The ACF Field Generator CLI is a Rust-based command-line tool designed to expedite the process of generating boilerplate PHP templates from a JSON representation created by Advanced Custom Fields (ACF). This tool aims to reduce the time and effort spent on the repetitive task of translating ACF field configurations into PHP code.

## Features

- **JSON to PHP Conversion:** Converts ACF field configurations stored in JSON format into corresponding wordpress boilerplate.
  
- **Automatic Flex Content Layouts** Create the template parts for your layouts with all the fieldsin the right place.
- **Overwrite Flag** Files will not be overwritten unless specified in the command

### Options

- `--input`: Path to the input JSON file containing ACF field configurations.
  
- `--dest`: Path where the generated code will be created.

- `--overwrite`: if true existing filse will be overwritten. false by defualt



### Usage


1. clone repository
```shell
    git clone git@github.com:willnjl/acf-layout-gen.git
```
2. install executable (Unix)
```shell
    cargo build && sudo cp target/debug/acf-layout-gen /usr/local/bin
```
3.  Run your Rust CLI app from anywhere on your machine.

```shell
   acf-layout-gen --src inc/acf-json/group_654b6e4a8a06e.json --dest template-parts/blocks 
```

### Development
```shell 
cargo watch -s 'cargo run -- --src ./data.json --dest ./blocks -o
```  

