use crate::{command::Command, CommandMap};
use serenity::prelude::RwLock;
use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommand,
    prelude::TypeMapKey,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Framework<'a> {
    pub commands: CommandMap<'a>,
}

impl<'a> Framework<'a> {
    #[must_use]
    pub fn new() -> Self {
        Framework {
            commands: HashMap::new(),
        }
    }

    pub async fn add_command(&mut self, ctx: &Context, command_ptr: fn() -> Command<'a>) {
        let cmd = command_ptr();

        if !self.commands.contains_key(cmd.name) {
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command.name(cmd.name).description(cmd.description)
            })
            .await
            .expect("Unable to create command");

            self.commands.insert(cmd.name, cmd);
        }
    }
}

impl TypeMapKey for Framework<'static> {
    type Value = RwLock<Framework<'static>>;
}
