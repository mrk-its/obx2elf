mod consts;
mod obx;

use clap::Parser;
use env_logger::{Builder, Env};
use std::fs::File;
extern crate target_lexicon;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    in_file: String,
    out_file: String,
    #[clap(short, long)]
    align: Option<u64>,
    #[clap(short, long, default_value = "error")]
    log_level: String,
}

fn main() {
    let args = Args::parse();
    Builder::from_env(Env::default().default_filter_or(&format!("obx2elf={}", args.log_level)))
        .init();

    let mut in_file = File::open(&args.in_file).expect("opened_file");
    let obx = obx::Obx::parse(&mut in_file);
    obx.to_elf(&args.in_file, args.align)
        .write(File::create(&args.out_file).unwrap())
        .unwrap();
}
