use anyhow::{anyhow, bail};
use structopt::StructOpt;
use serde_yaml;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug)]
enum GenerateStatus {
    Generated,
    Printed,
}

#[derive(StructOpt, Debug)]
pub struct Nixy {
    /// Undo the previous action 
    #[structopt(short, long)]
    undo: bool,
    /// Generate a template for you to fill in 
    #[structopt(short, long)]
    generate: bool,
    /// Run without actually updating anything
    #[structopt(short, long)]
    dryrun: bool,
}

// #[derive(Debug, PartialEq, Default)]
// struct NixTemplate {
//     tools: Vec
// }

pub fn generate_template() {
    // let nixTemplate: &'static str = "the quick brown fox jumps over the lazy dog";

    // let buildInput = format!("pkgs.{}");

    // let a = "AAA";
    // let b = format!("BBB {}", a);
    unimplemented!()
}

pub fn run(api_key: String, args: Nixy) -> anyhow::Result<()> {
    let f = std::fs::File::open("nixy.yaml")?;
    let d: String = serde_yaml::from_reader(f)?;
    let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&d)?;
    println!("{:?}", deserialized_map);

    let Nixy { generate, dryrun, undo } = args;

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