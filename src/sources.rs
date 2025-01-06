use crate::agent_impl::AIAssistant;
use crate::browser_impl::BrowserSearch;
use crate::cli::CLIArgs;

const BING: &'static str = "https://www.bing.com/search"; // ?q=top+gun
const DDG: &'static str = "https://duckduckgo.com/"; // ?q=top+gun
const GOOGLE: &'static str = "https://www.google.com/search"; // ?q=top+gun

pub enum HelperType {
    AIAssistant(AIAssistant),
    BrowserSearch(BrowserSearch)
}

pub fn set_source(args: &CLIArgs) -> HelperType {
    let source = if args.gpt {
        HelperType::AIAssistant( AIAssistant{
            query: args.query.clone()
        })
    } else {
        let engine = set_search_engine(args);
        HelperType::BrowserSearch( BrowserSearch {
            engine: engine.to_string(), 
            query: args.query.clone()
        })
    };
    return source;
}

pub fn set_search_engine(args: &CLIArgs) -> &str{
    let engine: &str = match (args.bg, args.ddg, args.gg) {
        (true, _, _) => BING,
        (_, true, _) => DDG,
        (_, _, true) => GOOGLE,
        // pattern never reached
        _ => GOOGLE
    };
    return engine;
}

