
use clap::Clap;

#[derive(Clap)]
pub enum ConfigSubCommand {
    Delete(Delete),
    Print(Print),
    Update(Update)
}

#[derive(Clap)]
pub struct Delete {}

#[derive(Clap)]
pub struct Print {
    /// Prints entire configuration file, including Last.fm API key
    #[clap(short = 'f', takes_value = false)]
    pub full_config: bool,
}

#[derive(Clap)]
pub struct Update {}
