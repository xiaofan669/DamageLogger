use std::{env, fmt::Display, fs, str::FromStr};

use cargo_metadata::MetadataCommand;
use regex::Regex;

struct Patch {
    major: usize,
    minor: usize,
    revision: usize,
}

impl Display for Patch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revision)
    }
}

fn main() {
    let patch_re = Regex::new(r"(\d+).(\d+).(\d+)").unwrap();

    // Update cargo metadata
    if let Ok(update) = env::var("UPDATE") {
        let cargo_toml_path = "Cargo.toml";
        let toml_str = fs::read_to_string(cargo_toml_path).unwrap();
        let mut doc = toml_edit::DocumentMut::from_str(&toml_str).unwrap();
        if let Some(metadata) = doc["package"]["metadata"].as_table_mut() {
            if let Some(capture) = patch_re.captures(metadata["patch"].as_str().unwrap()) {
                let (_full, [major_patch, minor_patch, revision_patch]) = capture.extract();
                let mut patch = Patch {
                    major: major_patch.parse::<usize>().unwrap(),
                    minor: minor_patch.parse::<usize>().unwrap(),
                    revision: revision_patch.parse::<usize>().unwrap(),
                };

                match update.as_str() {
                    "MAJOR_PATCH" => {
                        patch.major += 1;
                        patch.minor = 0;
                        patch.revision = 0;
                        metadata["hotfix"] = toml_edit::value("0.1");
                    }
                    "MINOR_PATCH" => {
                        patch.minor += 1;
                        patch.revision = 0;
                        metadata["hotfix"] = toml_edit::value("0.1");
                    }
                    "REVISION_PATCH" => {
                        patch.revision += 1;
                        metadata["hotfix"] = toml_edit::value("0.1");
                    }
                    "HOTFIX_PATCH" => {
                        let hotfix =
                            metadata["hotfix"].as_str().unwrap().parse::<f32>().unwrap() + 0.1;
                        metadata["hotfix"] = toml_edit::value(hotfix.to_string());
                    }
                    _ => {}
                }
                metadata["patch"] = toml_edit::value(patch.to_string());
            }
        }
        fs::write(cargo_toml_path, doc.to_string()).unwrap();
    }

    let metadata_cmd = MetadataCommand::new().exec().unwrap();

    let metadata = &metadata_cmd.root_package().unwrap().metadata;

    let region = metadata["region"].as_str().unwrap();
    let patch = metadata["patch"].as_str().unwrap();
    let hotfix = metadata["hotfix"].as_str().unwrap();

    let build_metadata = if metadata["is_beta"].as_bool().unwrap() {
        format!("{region}-beta-{patch}-{hotfix}")
    } else {
        format!("{region}-prod-{patch}-{hotfix}")
    };
    println!("cargo:rustc-env=TARGET_BUILD={build_metadata}");

    winres::WindowsResource::new()
        .set("FileDescription", &build_metadata)
        .compile()
        .unwrap();
}
