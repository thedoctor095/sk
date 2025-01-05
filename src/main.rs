use browser_impl::SearchBrowserQuery;
use agent_impl::PromptAI;
use clap::Parser;
use cli::CLIArgs;
use sources::{set_source, HelperType};

mod agent_impl;
mod browser_impl;
mod cli;
mod sources;

fn main() {
    let args = CLIArgs::parse();
    let source = set_source(&args);
    
    match source {
        HelperType::AIAssistant(ref assistant) => assistant.run(),
        HelperType::BrowserSearch(ref browser) =>browser.run()
    }
}
