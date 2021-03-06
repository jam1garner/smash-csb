use csb::CsbFile;
use structopt::StructOpt;

use std::path::{Path, PathBuf};
use std::fs;

#[derive(StructOpt)]
#[structopt(about = "A tool for converting between Smash Ultimate common sound table files and yaml")]
struct Args {
    in_file: PathBuf,
    out_file: PathBuf,

    #[structopt(short, long, help = "newline-separated hash labels to use")]
    labels: Option<PathBuf>,
}

fn main() {
    let args = Args::from_args();

    match CsbFile::open(&args.in_file) {
        Ok(bgm_prop_file) => {
            let _ = csb::hash40::set_custom_labels(
                csb::hash40::read_custom_labels(
                    args.labels.as_deref().unwrap_or(Path::new("ParamLabels.csv"))
                ).unwrap()
                .into_iter()
            );

            fs::write(&args.out_file, serde_yaml::to_string(&bgm_prop_file).unwrap()).unwrap();
        }
        Err(csb::Error::BadMagic { .. }) => {
            // Magic doesn't match, should be yaml file

            let contents = fs::read_to_string(&args.in_file).unwrap();
            let bgm_prop_file: CsbFile = serde_yaml::from_str(&contents).unwrap();

            bgm_prop_file.save(&args.out_file).unwrap();
        },
        Err(err) => {
            // Another error occurred, magic matches but failed to parse
            eprintln!("An error occurred: {}", err);
        }
    }
}
