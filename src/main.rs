use std::path::PathBuf;

use clap::Parser;
use pslr::PublicSuffixList;

const BUILT_IN_PSL: &[u8] = include_bytes!("../meta/public_suffix_list.dat");

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(help = "A list of names", required = true)]
    names: Vec<String>,

    #[clap(short, long, help = "Print the suffix for each name")]
    suffix: bool,

    #[clap(short, long, help = "Print the registrable domain for each name")]
    domain: bool,

    #[clap(short, long, help = "Path to public suffix list (uses the built-in list if not specified)")]
    list: Option<PathBuf>,
}

fn main() {
    let args = Arguments::parse();
    let psl = if let Some(data) = args.list {
        PublicSuffixList::try_from(data.as_path())
    } else {
        PublicSuffixList::try_from(BUILT_IN_PSL)
    };

    let Ok(psl) = psl else {
        eprintln!("Failed to load Public Suffix List data.");
        std::process::exit(1);
    };

    for name in args.names {
        if args.suffix {
            match psl.suffix(name.as_bytes()) {
                Some(suffix) => println!("{}", String::from_utf8_lossy(suffix)),
                None => {
                    eprintln!("no suffix found for {name}");
                    std::process::exit(1);
                },
            }
        } else if args.domain {
            match psl.domain(name.as_bytes()) {
                Some(domain) => println!("{}", String::from_utf8_lossy(domain)),
                None => {
                    eprintln!("no registrable domain found for {name}");
                    std::process::exit(1);
                },
            }
        } else {
            println!("{name}")
        }
    }
}
