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
sk <OPTIONAL_ARG> <QUERY_STRING>
  Optional args
  -ddg              Uses DuckDuckGo search engine
  -g (default)      Uses Google search engine
  -b                Uses Bing search engine
  
  Query string
  Example: weather today in Oslo

  Note: if two optional args are provided, the later will be considered part of the query
```

# Examples

**1. Basic usage**
```
./target/release/sk "weather today in Oslo"
```
**2. Specify Search Engine**
```
./target/release/sk -ddg "weather today in Oslo"
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

This project uses the [url](https://crates.io/crates/url) & [webbrowser](https://crates.io/crates/webbrowser) crates.

Enjoy searching!
