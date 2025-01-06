# Overview
<p align="center">
    <a href="https://opensource.org/licenses/MIT" alt="MIT License">
        <img src="https://img.shields.io/badge/License-MIT-orange.svg" /></a>
    <a href="https://www.rust-lang.org/tools/install" alt="Rust 1.83.0">
        <img src="https://img.shields.io/badge/Rust-1.83.0-orange.svg" /></a>
</p>
<p align="center">
    <strong>
    Built for the ultimate comfort while coding!
    </strong>
</p>

Sk (Seek) is a command-line interface (CLI) tool written in Rust that lets you either search the web using a search engine or get responses from ChatGPT, depending on your needs, all from the terminal.

# Features

Incorporates multiple search engines: Bing, DuckDuckGo or Google.

Query ChatGPT for blazing fast responses tailored to assist during coding.

Quickly launch searches from the command line without the need to manually open a browser.

# Prerequisites
Make sure **Rust** is installed on your system - the tool was tested on **MacOS Ventura 13.3.1** and **Ubuntu 22.04.3 LTS**

# Installation

To use Sk, follow these steps:

**1. Clone the Repository**

```
git clone git@github.com:thedoctor095/sk.git
```

**2. Build the Project**

```
cd sk
cargo build --release
```

**3. Run the Application**
```
./target/release/sk "Your search query"
```

# Usage
```
    Built for the ultimate comfort while coding!

    A command line tool that lets you either search the web using a search engine or get responses from ChatGPT, depending on your needs, all from the terminal.
    

Usage: sk <--bg|--ddg|--gg|--gpt> <QUERY>...

Arguments:
  <QUERY>...  Query to be searched using a search engine or to prompt ChatGPT

Options:
      --bg       Sets search engine to Bing
      --ddg      Sets search engine to DuckDuckGo
      --gg       Sets search engine to Google
      --gpt      Prompts ChatGPT model gpt-4o-mini
  -h, --help     Print help
  -V, --version  Print version

```

# Examples

**1. Basic usage**
```
./target/release/sk --ddg "weather today in Oslo"

Opening https://duckduckgo.com/?q=weather+today+in+oslo
```
**2. Query ChatGPT**
```
./target/release/sk --gpt factorial implementation in rust

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let num = 5;
    println!("Factorial of {} is {}", num, factorial(num));
}
```
**3. Help Menu**
```
./target/release/sk --help
```

# Contributing

Feel free to contribute to Sk by opening issues, submitting pull requests, or suggesting improvements.

# License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/thedoctor095/sk/blob/master/LICENSE) file for details.

# Acknowledgments

This project uses [clap](https://crates.io/crates/clap), [openai-api-rs](https://crates.io/crates/openai-api-rs), [tokio](https://crates.io/crates/tokio), [url](https://crates.io/crates/url) & [webbrowser](https://crates.io/crates/webbrowser)crates.

Enjoy searching!
