use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::common::GPT4_O_MINI;
use openai_api_rs::v1::chat_completion::{
    ChatCompletionMessage, ChatCompletionRequest, ChatCompletionResponse, Content, MessageRole};
use std::env;
use std::process;
use tokio::runtime::Runtime;

const PROMPT: &'static str = "Answer strictly with code or websites unless asked to explain or describe. Format answers as clean, plain-text paragraphs, no Markdown, HTML, or special syntax. Keep it terminal-friendly.";

pub trait PromptAI {
    fn run(&self) -> ();

    fn build(&self, api_key: String) -> OpenAIClient;

    fn build_prompt(&self) -> Vec<ChatCompletionMessage>;

    fn build_request(&self) -> ChatCompletionRequest;

    async fn fetch_gpt_request(
        &self, 
        client: OpenAIClient, 
        request: ChatCompletionRequest) -> ChatCompletionResponse;

    fn get_api_key(&self) -> String;

    fn get_runtime(&self) -> Runtime;
}

pub struct AIAssistant {
    pub query: Vec<String>
}

impl PromptAI for AIAssistant {
    fn run(&self) -> () {
        let api_key = self.get_api_key();
        let client = self.build(api_key);
        let request = self.build_request();
        let async_runtime = self.get_runtime();
        let response = async_runtime.block_on(
            async {
                return self.fetch_gpt_request(client, request).await;
            }
        );
        let response_text = &response.choices[0].message.content.as_ref().unwrap();
        println!("{}", response_text)
    }

    fn build(&self, api_key: String) -> OpenAIClient {
        match OpenAIClient::builder().with_api_key(api_key).build() {
            Ok(value) => return value,
            Err(_) => {
                println!("Could not build OpenAIClient");
                process::exit(1);
            }
        };
    }

    fn build_prompt(&self) -> Vec<ChatCompletionMessage> {
        let query = self.query.join(" ");
        let system_role = ChatCompletionMessage {
            role: MessageRole::system,
            content: Content::Text(String::from(PROMPT)),
            name: None,
            tool_calls: None,
            tool_call_id: None
        };
        let user_role = ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text(String::from(query)),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        };
        return vec![system_role, user_role]
    }

    fn build_request(&self) -> ChatCompletionRequest {

        let message = self.build_prompt();
        let request = ChatCompletionRequest::new(
            GPT4_O_MINI.to_string(),
            message,
        );
        return request;
    }

    async fn fetch_gpt_request(
        &self, 
        client: OpenAIClient, 
        request: ChatCompletionRequest) -> ChatCompletionResponse{
        match client.chat_completion(request).await {
            Ok(response) => return response,
            Err(e) => {
                eprintln!("Could not fetch query result\n {}",e.to_string());
                process::exit(1);
            }
        }
    }

    fn get_api_key(&self) -> String {
        let api_key = match env::var("OPENAI_API_KEY") {
            Ok(value) => value,
            Err(_) => {
                println!("Missing OPENAI_API_KEY");
                process::exit(1)
            }};
        return api_key;
        }
    
    fn get_runtime(&self) -> Runtime {
        match Runtime::new() {
            Ok(value) => return value,
            Err(e) => {
                eprintln!("Error creating Tokio runtime {:?}", e);
                process::exit(1);
            }
        };
    }
}