use clap::{ArgGroup, Parser};


#[derive(Parser, Debug)]
#[command(
    name = "Seek (Sk)",
    version = "0.2.1",
    about = "
    Built for the ultimate comfort while coding!

    A command line tool that lets you either search the web using a search engine or get responses from ChatGPT, depending on your needs, all from the terminal.
    ",
    group(
    ArgGroup::new("engines")
        .required(true)
        .args(&["bg", "ddg", "gg", "gpt"])
))]
pub struct CLIArgs {
    #[clap(long = "bg", help = "Sets search engine to Bing")]
    pub bg: bool,
    #[clap(long = "ddg", help = "Sets search engine to DuckDuckGo")]
    pub ddg: bool,
    #[clap(long = "gg", help = "Sets search engine to Google")]
    pub gg: bool,
    #[clap(long = "gpt", help = "Prompts ChatGPT model gpt-4o-mini")]
    pub gpt: bool,
    #[clap(required = true, help = "Query to be searched using a search engine or to prompt ChatGPT")]
    pub query: Vec<String>
}
