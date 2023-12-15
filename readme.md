# ACF Field Generator CLI

## Overview

The ACF Field Generator CLI is a Rust-based command-line tool designed to expedite the process of generating boilerplate PHP templates from a JSON representation created by Advanced Custom Fields (ACF). This tool aims to reduce the time and effort spent on the repetitive task of translating ACF field configurations into PHP code.

## Features

- **JSON to PHP Conversion:** Converts ACF field configurations stored in JSON format into corresponding wordpress boilerplate.
  
- **Customizable Output:** Allows users to specify output preferences, such as the file name, directory, and other relevant options.

- **Time Savings:** Automates the generation of PHP boilerplate code, saving developers significant time and effort.

### Options

- `--input`: Path to the input JSON file containing ACF field configurations.
  
- `--output`: Path to the output PHP file where the generated code will be saved.

- (Optional) Additional options to customize the output.



### Usage


1. clone repository
    `git clone git@github.com:willnjl/flex-gen-acf.git`
2. install executable (Unix)
    `cargo build && sudo cp target/debug/flex-gen-acf /usr/local/bin`
3. Run your Rust CLI app from anywhere on your machine.
   `flex-gen-acf`

### Development
```shell 
cargo watch -s 'cargo run -- --src ./data.json --dest ./blocks -o
```  

