use ds_api::{Message, Response as _, Role};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub new_favorability: i32,
    pub current_emotion: String,
    pub response: String,
    pub mind: String,
}

#[allow(clippy::too_many_arguments)]
pub async fn chat(
    token: &str,
    emotion: String,
    favorability: i32,
    character_design: String,
    response_requirement: String,
    character_emotion_split: String,
    model: String,
    temperature: Option<f64>,
    max_tokens: Option<i32>,
    _title: String,
    mut messages: Vec<Message>,
    content: String,
) -> Result<(Response, Message), String> {
    let model = match model.as_str() {
        "deepseek-chat" => ds_api::Model::DeepseekChat,
        "deepseek-reasoner" => ds_api::Model::DeepseekReasoner,
        _ => return Err("无此模型".to_string()),
    };

    let system_prompt = generate_system_prompt(
        emotion,
        favorability,
        character_design,
        response_requirement,
        character_emotion_split,
    );

    let mut c_messages = vec![Message::new(Role::System, &system_prompt)];
    c_messages.append(&mut messages);

    c_messages.push(Message::new(Role::User, &content));

    let mut request = ds_api::Request::builder().messages(c_messages).model(model);

    if let Some(temperature) = temperature {
        request = request.temperature(temperature as f32);
    }

    if let Some(max_tokens) = max_tokens {
        request = request.max_tokens(max_tokens as u32);
    }

    let response = request
        .execute_nostreaming(token)
        .await
        .map_err(|e| e.to_string())?;

    Ok((
        serde_json::from_str(response.content()).map_err(|e| e.to_string())?,
        response.choices[0].message.clone(),
    ))
}

fn generate_system_prompt(
    emotion: String,
    favorability: i32,
    character_design: String,
    response_requirement: String,
    character_emotion_split: String,
) -> String {
    let lines = character_emotion_split
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().to_string())
        .collect::<Vec<_>>();

    let emotion_description = if let Some(x) = lines.iter().find(|x| {
        x.split(" ")
            .next()
            .unwrap()
            .split("..=")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| favorability >= w[0] && favorability <= w[1])
    }) {
        x.split(" ").nth(2).unwrap().to_string()
    } else {
        "".to_string()
    };

    format!(
        "{}\n你当前的情绪是:{}\n你当前的好感度是:{}\n{}\n{}",
        character_design, emotion, favorability, emotion_description, response_requirement
    )
}
