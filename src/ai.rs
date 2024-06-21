use openai_api_rust::chat::*;
use openai_api_rust::*;
use std::env;

pub fn choose_ticket() -> String {
    let auth = Auth::new(&env::var("AI_KEY").unwrap());
    let openai = OpenAI::new(auth, "***REMOVED***");
    let body = ChatBody {
        model: "llama3".to_string(),
        max_tokens: Some(256),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![
            Message {
                role: Role::User,
                content: "Hello!".to_string(),
            },
        ],
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();

    message.content.clone()
}
