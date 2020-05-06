use clap::Clap;
use entropy::Entropy;
use std::fs::File;

#[derive(Clap)]
#[clap(
    version = "1.0.2",
    author = "tux <me@johnpacific.com>",
    about = "A utility to calculate Shannon entropy of a given file"
)]
struct Opts {
    #[clap(about = "The target file to measure")]
    filepath: String,
    #[clap(short, long, about = "Returns metric entropy instead of Shannon entropy")]
    metric_entropy: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let file = File::open(opts.filepath).expect("Couldn't open the file at the provided path.");
    let entropy_calc = Entropy::new(&file);

    // --metric-entropy
    if opts.metric_entropy {
        println!("{}", entropy_calc.metric_entropy());
    } else {
        println!("{}", entropy_calc.shannon_entropy());
    }
}
