use std::fs::create_dir_all;

use serde::Serialize;

#[derive(Serialize)]
struct SampleOption {
    sub_option: String,
}

#[derive(Serialize)]
struct SampleConfig {
    config_path: std::path::PathBuf,
    output_directory: std::path::PathBuf,
    option: SampleOption,
}

#[derive(Serialize)]
struct Options {
    config_option: String,
    configs: Vec<SampleConfig>,
}

fn main() {
    println!("Generating a bunch of config formats");

    let options = {
        Options {
            config_option: "my_option".into(),
            configs: vec![
                SampleConfig {
                    config_path: "subdir".into(),
                    output_directory: "directory/subdir".into(),
                    option: SampleOption {
                        sub_option: "option1".into(),
                    },
                },
                SampleConfig {
                    config_path: "subdir2".into(),
                    output_directory: "directory/subdir2".into(),
                    option: SampleOption {
                        sub_option: "option2".into(),
                    },
                },
            ],
        }
    };

    create_dir_all("out").unwrap();
    serde_any::to_file_pretty("out/config.toml", &options).unwrap();
    serde_any::to_file_pretty("out/config.yaml", &options).unwrap();
    serde_any::to_file_pretty("out/config.json", &options).unwrap();
    serde_any::to_file_pretty("out/config.ron", &options).unwrap();
    // Doesn't seem to work?
    //serde_any::to_file_pretty("out/config.xml", &options).unwrap();
}
