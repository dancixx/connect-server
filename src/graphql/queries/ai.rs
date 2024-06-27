use std::vec;

use async_graphql::{Context, FieldResult, Object};
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};

use crate::{
    graphql::types::{surreal_id::SurrealID, OpenAiClient, SurrealClient},
    models::{ai::AIBioResponse, users::User},
};

#[derive(Default)]
pub struct AiQueryRoot;

#[Object]
impl AiQueryRoot {
    #[graphql(name = "aio_bio_description")]
    pub async fn ai_bio_desription(
        &self,
        context: &Context<'_>,
        #[graphql(name = "user_id")] user_id: String,
    ) -> FieldResult<AIBioResponse> {
        let surreal = context.data::<SurrealClient>()?;
        let SurrealID(thing) = SurrealID::from(user_id);
        let user = surreal.select::<Option<User>>(thing).await?;
        tracing::info!("User: {:?}", user);
        let user = serde_json::to_string(&user).unwrap();

        let openai = context.data::<OpenAiClient>()?;
        // Write a kind and friendly bio about the user and use emojis
        let messages = vec![
            ChatCompletionRequestSystemMessageArgs::default()
                .content("Write a kind and friendly bio about the user and use emojis! The bio should not contain name, username, age, weight, or height-related information. Focus on the user's personality and interests. Use the user's language and tone. This bio should follow the guidelines of a dating app bio.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(format!("Please write a kind and friendly bio about me. You can use my database profile for additional information: {user}. The bio should be cheerful, kind, and interesting, without any political or religious references. Skip the name and physical characteristics. It should feel human and like an introduction to someone who already knows my name. Don't focus too much on things like eduction, job, everything should be a bit general and neutral."))
                .build()?
                .into(),
        ];
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o")
            .max_tokens(250_u32)
            .messages(messages)
            .build()?;
        let response = openai.chat().create(request).await?;
        let response = response.choices.first().unwrap();
        let response = response.message.content.clone().unwrap();

        Ok(AIBioResponse { bio: response })
    }
}
