use std::process;
use url::Url;
use webbrowser::open as browser;

pub trait SearchBrowserQuery {
    fn open_page(&self, page: Url) -> ();

    fn parse_url(&self, engine: &str, search_query: &Vec<String>) -> Url;

    fn run(&self) -> ();
}

pub struct BrowserSearch {
    pub engine: String,
    pub query: Vec<String>
}

impl SearchBrowserQuery for BrowserSearch {
    fn open_page(&self, page: Url) -> () {
        match browser(page.as_str()) {
            Ok(_) => println!("Opening {}", page),
            Err(err) => eprintln!("{}", err)
        }
    }

    fn parse_url(&self, engine: &str, search_query: &Vec<String>) -> Url {
        let search_query = search_query.join(" ");
        let new_url = match Url::parse_with_params(engine, &[("q", search_query)]) {
            Ok(parsed_engine) => parsed_engine,
            Err(err) => {
                eprintln!("Could not parse URL: {}", err);
                process::exit(0);
            }
        };
        return new_url
    }

    fn run(&self) -> () {
        let url = self.parse_url(&self.engine, &self.query);
        self.open_page(url);
    }
}