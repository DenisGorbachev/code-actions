* Fix to_module_token_stream not to use Config::default
* Fix ModuleTemplate::function to return a plain function (like before)
* Refactor the code to never generate a `type_name`, always use `&ident`
  * Introduce `get_matching_extras`
  * Remove duplicate `get_extra_derives_for_name`, `get_extra_uses_for_name`
