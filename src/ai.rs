use anyhow::{Context, Result};
use openai_api_rust::chat::*;
use openai_api_rust::*;
use std::env;

use crate::jira::Issues;

pub fn choose_ticket(diff: String, tickets: Issues) -> Result<String> {
    let diff_message = Message {
        role: Role::User,
        content: diff,
    };

    let instruction = Message {
        role: Role::System,
        content: "You are a devops engineer tasked with choosing a ticket number that matches most closely to the diff. You may choose only a single valid ticket name from the list provided.".to_string(),
    };

    let mut jira_ticket_content = String::new();

    for ticket in tickets {
        jira_ticket_content.push_str(&format!(
            "{}: {}\ndescription: {}\n",
            ticket.key, ticket.summary, ticket.description
        ));
    }

    let jira_message = Message {
        role: Role::User,
        content: jira_ticket_content,
    };

    let answer_prompt = Message {
        role: Role::User,
        content: "Please choose a ticket number from the list above. Justify why this specific ticket matches to the diff:".to_string(),
    };

    let auth = Auth::new(&env::var("AI_KEY").context("AI_KEY not defined")?);
    let openai = OpenAI::new(auth, &env::var("AI_HOST").context("AI_HOST not defined")?);
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
        messages: vec![instruction, diff_message, jira_message, answer_prompt],
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();

    log::debug!("AI response: {:?}", message);

    let ticket_regex = regex::Regex::new(&env::var("TICKET_REGEX").context("TICKET_REGEX not defined")?)?;

    let ticket = ticket_regex.captures(&message.content).context("No ticket number found")?;

    Ok(ticket.get(0).unwrap().as_str().to_string())
}
