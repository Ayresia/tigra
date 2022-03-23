use serenity::{
    builder::CreateApplicationCommandOption,
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandOptionType,
    },
};

use crate::FnPtr;

pub struct Command<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub options: Vec<CreateApplicationCommandOption>,
    pub ptr: FnPtr,
}

impl<'a> Command<'a> {
    pub fn new(name: &'a str, description: &'a str, ptr: FnPtr) -> Self {
        Command {
            name,
            description,
            options: vec![],
            ptr,
        }
    }

    pub fn add_option(
        mut self,
        name: &'a str,
        description: &'a str,
        kind: ApplicationCommandOptionType,
        required: bool,
    ) -> Self {
        let mut option = CreateApplicationCommandOption::default();

        option
            .name(name)
            .description(description)
            .required(required)
            .kind(kind);

        self.options.push(option);
        self
    }

    pub async fn invoke(&self, ctx: &Context, interaction: ApplicationCommandInteraction) {
        (self.ptr)(ctx, interaction).await;
    }
}
