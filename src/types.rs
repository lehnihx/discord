pub struct Data;
pub struct CommandConfig {
    pub name: &'static str,
    pub description_key: &'static str,
    pub action: ActionFn,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ActionFn = for<'a> fn(Context<'a>) -> poise::BoxFuture<'a, Result<(), Error>>;
