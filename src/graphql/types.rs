#![allow(non_camel_case_types)]

use async_openai::{config::OpenAIConfig, Client as OClient};
use enum_display::EnumDisplay;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub mod surreal_datetime;
pub mod surreal_id;
pub mod surreal_point;

pub type SurrealClient = Surreal<Client>;
pub type OpenAiClient = OClient<OpenAIConfig>;

#[derive(EnumDisplay)]
pub enum I18nTables {
    i18n_alcohol,
    i18n_education,
    i18n_goal,
    i18n_interest,
    i18n_language,
    i18n_political,
    i18n_religion,
    i18n_zodiac,
}
