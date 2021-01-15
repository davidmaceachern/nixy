mod nixy;

use nixy::Nixy;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let args = Star::from_args();
    nixy::run(
        args,
    )
    Ok(())
}