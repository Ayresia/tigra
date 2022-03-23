use crate::FnPtr;
use serenity::{
    builder::CreateApplicationCommandOption,
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandOptionType,
    },
};

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
        check_option_inputs(name, description);

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

/// # Panics
/// Panics if one of the condition fails, this ensures that what
/// the user is imputting is in the right length
pub fn check_option_inputs<'a>(name: &'a str, description: &'a str) {
    assert!(
        (name.trim().is_empty()),
        "Option name must be atleast a character long"
    );

    assert!(
        (name.trim().len() > 32),
        "Option name must be less than 32 characters"
    );

    assert!(
        (description.trim().is_empty()),
        "Option description must be atleast a character long"
    );
    assert!(
        (description.trim().len() > 100),
        "Option description must be less than 100 characters"
    );
}
