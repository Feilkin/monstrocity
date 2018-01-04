// Commands for API methods, so we can do cool stuff

use super::Bot;

pub trait Command {
    fn execute(&self, bot: &Bot) -> ();
}
