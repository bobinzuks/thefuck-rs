use super::{Command, Rule};

pub struct DjangoSouthGhost;

impl Rule for DjangoSouthGhost {
    fn name(&self) -> &str { "django_south_ghost" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("django") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
