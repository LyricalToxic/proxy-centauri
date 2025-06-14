use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    
    #[arg(short, long, default_value = "7272")]
    pub(crate) port: u32,
}
