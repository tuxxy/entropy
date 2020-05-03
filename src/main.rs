use clap::Clap;

#[derive(Clap)]
#[clap(
    version = "0.1",
    author = "tux <me@johnpacific.com>",
    about = "A utility to calculate Shannon entropy"
)]
struct Opts {
    filename: String,
}

fn main() {
    let opts: Opts = Opts::parse();
}
