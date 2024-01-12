# Overview

Sk (Seek) is a command-line interface (CLI) application written in Rust that allows users to input any search query as argument and opens a browser tab to search the input using various search engines. It provides a convenient way to quickly look up information from different sources without manually opening a web browser.

# Features

Search Engines: Incorporates multiple search engines: DuckDuckGo / Bing / Google.

Efficiency: Quickly launch searches from the command line without the need to open a browser manually.

# Prerequisites
_Make sure **Rust** is installed on your system - this application was built & tested on **Ubuntu 22.04.3 LTS**_

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
Sk (Seek) 0.1.3
A command line app that opens a browser tab and searches for a query given a search engine argument (or not)

USAGE:
    sk [FLAGS] <search-query>...

FLAGS:
    -b, --bg         Sets search engine to Bing
    -d, --ddg        Sets search engine to DuckDuckGo
    -g, --gg         Sets search engine to Google (default)
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <search-query>...    The query to be searched with the selected search engine

```

# Examples

**1. Basic usage**
```
./target/release/sk "weather today in Oslo"
```
**2. Specify Search Engine**
```
./target/release/sk --ddg "weather today in Oslo"
```
**3. Help Menu**
```
./target/release/sk
```

# Contributing

Feel free to contribute to Sk by opening issues, submitting pull requests, or suggesting improvements.

# License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/thedoctor095/sk/blob/master/LICENSE) file for details.

# Acknowledgments

This project uses the [url](https://crates.io/crates/url), [webbrowser](https://crates.io/crates/webbrowser) & [structopt](https://docs.rs/structopt/latest/structopt/) crates.

Enjoy searching!
