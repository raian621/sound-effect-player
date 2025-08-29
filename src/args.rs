use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub config_path: Option<String>,
}

impl Args {
    pub fn get_args() -> Args {
        Args::parse()
    }
}
