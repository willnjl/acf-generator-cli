use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// src file containing acf fields (JSON)
    #[arg(short, long)]
    pub src: String,

    /// destination folder
    #[arg(short, long, default_value = "./")]
    pub dest: String,

    #[arg(short, long)]
    pub overwrite: bool,
}
