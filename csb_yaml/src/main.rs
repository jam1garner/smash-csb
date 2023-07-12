use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use csb_lib::{CsbFile, Hash40};

/// Convert commonsoundtable.csb files to and from YAML
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input CSB or YAML file path
    pub input: String,
    /// The output CSB or YAML file path
    pub output: Option<String>,
    /// The labels file path for pairing hashes with strings
    pub label: Option<String>,
}

fn read_data_write_yaml(
    input_path: String,
    output_path: Option<String>,
    label_path: Option<String>,
) {
    let output_path = output_path
        .map(|o| PathBuf::from(&o))
        .unwrap_or_else(|| PathBuf::from(&(input_path.to_string() + ".yaml")));

    match CsbFile::from_file(input_path) {
        Ok(csb) => {
            let label_path = PathBuf::from(label_path.unwrap_or("Labels.txt".to_owned()));

            if label_path.is_file() {
                let label_clone = Hash40::label_map();
                let mut labels = label_clone.lock().unwrap();

                labels.add_labels_from_path(label_path).unwrap();
            }

            let yaml = serde_yaml::to_string(&csb).unwrap();

            fs::write(output_path, yaml).expect("failed to write YAML file");
        }
        Err(error) => eprintln!("{error:?}"),
    }
}

fn read_yaml_write_data<P: AsRef<Path>>(input_path: P, output_path: Option<String>) {
    let yaml = fs::read_to_string(&input_path).unwrap();

    match serde_yaml::from_str::<CsbFile>(&yaml) {
        Ok(csb) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| input_path.as_ref().with_extension("csb"));

            csb.write_to_file(output_path)
                .expect("failed to write CSB file");
        }
        Err(error) => eprintln!("{error:?}"),
    }
}

fn main() {
    let args = Args::parse();

    match Path::new(&args.input)
        .extension()
        .expect("input file extension should exist")
        .to_str()
        .unwrap()
    {
        "yaml" | "yml" => read_yaml_write_data(args.input, args.output),
        _ => read_data_write_yaml(args.input, args.output, args.label),
    }
}
