// Copyright 2015, Yuheng Chen. See the LICENSE file at the top-level
// directory of this distribution.

//! Unity YAML parser.
//!
//! # Usage
//!
//! This crate is [on github](https://github.com/xmxu/unity-yaml-rust) fork from [yaml-rust](https://github.com/chyh1990/yaml-rust) and can be
//! used by adding `unity-yaml-rust` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! unity-yaml-rust = "0.1"
//! ```
//!
//! And this in your crate root:
//!
//!
//! Parse a string into `Vec<Yaml>` and then serialize it as a YAML string.
//!
//! # Examples
//!
//! ```
//! use unity_yaml_rust::{yaml::YamlLoader, emitter::YamlEmitter};
//!
//! let docs = YamlLoader::load_from_str("[1, 2, 3]").unwrap();
//! let doc = &docs[0]; // select the first document
//! assert_eq!(doc[0].as_i64().unwrap(), 1); // access elements by index
//!
//! let mut out_str = String::new();
//! let mut emitter = YamlEmitter::new(&mut out_str);
//! emitter.dump(doc).unwrap(); // dump the YAML object to a String
//!
//! ```

#![doc(html_root_url = "https://docs.rs/unity-yaml-rust/0.1.0")]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(match_same_arms, should_implement_trait)
)]

pub mod asset_file;
pub mod asset_types;
pub mod emitter;
pub mod parser;
pub mod scanner;
pub mod tools;
pub mod unity_document;
pub mod yaml;

// reexport key APIs
pub use crate::asset_file::AssetFile;
pub use crate::asset_types::AssetType;
pub use crate::emitter::{EmitError, YamlEmitter};
pub use crate::parser::Event;
pub use crate::scanner::ScanError;
pub use crate::tools::dump_node;
pub use crate::unity_document::UnityDocument;
pub use crate::yaml::{Yaml, YamlLoader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api() {
        let s = "
# from yaml-cpp example
- name: Ogre
  position: [0, 5, 0]
  powers:
    - name: Club
      damage: 10
    - name: Fist
      damage: 8
- name: Dragon
  position: [1, 0, 10]
  powers:
    - name: Fire Breath
      damage: 25
    - name: Claws
      damage: 15
- name: Wizard
  position: [5, -3, 0]
  powers:
    - name: Acid Rain
      damage: 50
    - name: Staff
      damage: 3
";
        let docs = YamlLoader::load_from_str(s).unwrap();
        let doc = &docs[0];

        assert_eq!(doc[0]["name"].as_str().unwrap(), "Ogre");

        let mut writer = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut writer);
            emitter.dump(doc).unwrap();
        }

        assert!(!writer.is_empty());
    }

    fn try_fail(s: &str) -> Result<Vec<Yaml>, ScanError> {
        let t = YamlLoader::load_from_str(s)?;
        Ok(t)
    }

    #[test]
    fn test_fail() {
        let s = "
# syntax error
scalar
key: [1, 2]]
key1:a2
";
        assert!(YamlLoader::load_from_str(s).is_err());
        assert!(try_fail(s).is_err());
    }
}
