use super::{Command, Rule};

pub struct DjangoSouthMerge;

impl Rule for DjangoSouthMerge {
    fn name(&self) -> &str { "django_south_merge" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("django") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
