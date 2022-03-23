use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};
use std::{collections::HashMap, future::Future, pin::Pin};

pub use macros;
pub mod command;
pub mod framework;

pub type CommandMap<'a> = HashMap<&'a str, command::Command<'a>>;
pub type BoxFuture<'a> = Pin<Box<dyn Future<Output = ()> + 'a + Send>>;
pub type FnPtr = for<'a> fn(&'a Context, ApplicationCommandInteraction) -> BoxFuture<'a>;
