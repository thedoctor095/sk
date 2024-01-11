use webbrowser;

use std::env;
use std::process;
use url::Url;

const GOOGLE: &'static str = "https://www.google.com/search"; //q=top+gun
const DDG: &'static str = "https://duckduckgo.com/"; // ?q=top+gun
const BING: &'static str = "https://www.bing.com/search"; // ?q=top+gun

fn open_page(page: Url){
    match webbrowser::open(page.as_str()) {
        Ok(_) => println!("Opening {}", page),
        Err(err) => eprintln!("Encountered {}", err)
    }
}

fn parse_url(args: Vec<String>) -> Url {
    let mut sk_engine: &str = &args[1];
    let mut has_engine_arg: bool = false;
    if sk_engine.starts_with("-") {
        sk_engine = match sk_engine {
            "-ddg" => DDG,
            "-g" => GOOGLE,
            "-b" => BING,
            _ => {
                eprintln!("Invalid argument {}, aborting", sk_engine);
                process::exit(0);
            }
        };
        has_engine_arg = true;
    } else {
        sk_engine = GOOGLE
    }
    let query: String;
    query = if has_engine_arg {args[2..].join(" ")} else {args[1..].join(" ")};
    let mut new_url: Url = Url::parse(sk_engine).unwrap();
    new_url.query_pairs_mut().append_pair("q", &query);
    return new_url
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE\n\
                  sk <OPTIONAL_ARG> <QUERY_STRING>\n\
                  Optional args\n\
                  -ddg              Uses DuckDuckGo search engine\n\
                  -g (default)      Uses Google search engine\n\
                  -b                Uses Bing search engine\n\
                  Query string\n\
                  Example: weather today in Oslo\n\
                  Note: if two optional args are provided, the later will be considered part of the query");
        println!("No arguments given, aborting..");
        process::exit(0);
    }
    else {
        let parsed_url: Url = parse_url(args);
        open_page(parsed_url);
    }

}