use crate::FnPtr;
use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};

pub struct Command<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub ptr: FnPtr,
}

impl<'a> Command<'a> {
    pub fn new(name: &'a str, description: &'a str, ptr: FnPtr) -> Self {
        Command {
            name,
            description,
            ptr,
        }
    }

    pub async fn invoke(&self, ctx: &Context, interaction: ApplicationCommandInteraction) {
        (self.ptr)(ctx, interaction).await;
    }
}
