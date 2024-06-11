# Groq-Cli

[Groq-Cli](https://github.com/Akshad135/groq-cli) rewritten in Rust.

## Downloads

- From releases (Binary File) [click here](https://github.com/Akshad135/groq-cli-rust/releases/tag/v0.1.0)
  - _Instructions to use the binary file is mentioned with the file_
- Or build it yourself:
  - Download the source code.
  - `cd` into the directory
  - In your terminal run `cargo build && cargo install --path .`
  - _Note: Building requires Rust to be install and configured_

## Usage

- Run the cmd `gcli` after installing the package.
- It will ask the Api key. (You can get the api key from [GroqCloud](https://console.groq.com/keys) after registering for free)
- After entering the Api Key, select the model in the later step.

## Commands

- `gcli [-s | --setup | -h | --help]`
  - `-s | --setup` : Runs the setup config again to change the api key or model.
  - `-h | --help` : Runs the help command. <br>
    _Just running `gcli` will prompt for user query_
