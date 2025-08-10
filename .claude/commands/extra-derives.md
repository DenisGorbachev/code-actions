# Implement configurable extra derives

* Implement reading a `code-actions.toml` config file
  * Use `figment` crate
  * Use `adjoin` fn to recursively combine configs
  * Find configs starting from `anchor` to workspace root
  * Use regex syntax to match names
* Allow the user to specify extra derives and extra use statements in the config file
* Add these extra derives and use statements to every template function mentioned in @src/types/module_template.rs
* Example config:
  ```toml
  [[extra]]
  matches = "User.*"
  use = ["serde::{Serialize, Deserialize}"]
  derive = ["Serialize", "Deserialize", "Clone", "Debug", "PartialEq"]
  
  [[extra]]
  matches = "Error$"
  use = ["derive_more::Error"]
  derive = ["Error", "Clone", "Debug"]
  
  [[extra]]
  matches = ".*"
  # `use` key is optional (defaults to empty vec)
  # `derive` key is optional (defaults to empty vec)
  ```
