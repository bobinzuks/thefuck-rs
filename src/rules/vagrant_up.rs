pub struct VagrantUp;

impl Rule for VagrantUp {
    fn name(&self) -> &'static str {
        "vagrant_up"
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.to_lowercase().contains("run `vagrant up`")
    }

    fn fix(&self, command: &Command) -> Option<String> {
        let cmds = &command.text.split_whitespace().collect::<Vec<&str>>();
        let machine = cmds.get(2).copied();
        
        let start_all = format!("vagrant up && {}", command.text);
        
        match machine {
            Some(m) => Some(format!("vagrant up {} && {}", m, command.text)),
            None => Some(start_all),
        }
    }
}