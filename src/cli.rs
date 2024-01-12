use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "Sk (Seek)",
about = "A command line app that opens a browser tab and searches for a query \
        given a search engine argument (or not)")]
pub struct CLIArgs {
    /// Sets search engine to DuckDuckGo
    #[structopt(short = "d", long = "ddg", global=false)]
    pub ddg: bool,
    /// Sets search engine to Bing
    #[structopt(short = "b", long = "bg", global=false)]
    pub bg: bool,
    /// Sets search engine to Google (default)
    #[structopt(short = "g", long = "gg", global=true)]
    pub gg: bool,
    /// The query to be searched with the selected search engine
    #[structopt(required = true)]
    pub search_query: Vec<String>,
}
