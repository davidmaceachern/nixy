// #![forbid(unsafe_code)]
// #![deny(missing_debug_implementations, nonstandard_style)]
// #![warn(future_incompatible, rust_2018_idioms)]
// #![warn(missing_docs)]

mod nixy;

use nixy::Nixy;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let args = Nixy::from_args();
    nixy::run(
        args,
    )
    Ok(())
}