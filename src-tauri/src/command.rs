#[derive(Debug)]
pub enum CommandType {
  Console(String), // In game console command
  LoadVdm(String)
}

#[derive(Debug)]
pub struct Command {
  pub demo_name: String,
  pub command_type: CommandType,
  // pub event: Event,
  pub tick: i64
}