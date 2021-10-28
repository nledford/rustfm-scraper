use clap::Parser;

#[derive(Parser)]
pub enum ConfigSubCommand {
    Delete(Delete),
    Print(Print),
    Update(Update),
}

#[derive(Parser)]
pub struct Delete {}

#[derive(Parser)]
pub struct Print {
    /// Prints entire configuration file, including Last.fm API key
    #[clap(short = 'f', takes_value = false)]
    pub full_config: bool,
}

#[derive(Parser)]
pub struct Update {}
