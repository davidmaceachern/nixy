use anyhow::{anyhow, bail};
use structopt::StructOpt;
use serde_yaml;

#[derive(StructOpt, Debug)]
pub struct Nixy {
    /// Generate a template for you to fill in 
    #[structopt(short, long)]
    generate: bool,
    /// Run without actually updating account
    #[structopt(short, long)]
    dryrun: bool,
}

pub fn generate_template() {
    unimplemented!()
}

pub fn run(api_key: String, args: Star) -> anyhow::Result<()> {
    let f = std::fs::File::open("nixy.yaml")?;
    let d: String = serde_yaml::from_reader(f)?;
    let Nixy { generate, dryrun } = args;
    if generate {
        let template_file_name = "nixy.yaml";
        println!("Generating a template for you to start with in {}", template_file_name);
        generate_template();
    }
    if dryrun {
        println!("You enabled the dryrun flag.");
    }
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_compares_lists() {
//         let result = diff();
//         assert_eq!(result, expected);
//     }
// } 