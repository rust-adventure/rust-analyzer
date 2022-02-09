# Rust Analyzer in VSCode

rust-analyzer is a modular compiler frontend for the Rust language. It is a part of a larger rls-2.0 effort to create excellent IDE support for Rust.

This repo is a set of videos that explain what it is and how to use it in VSCode

## Overview

Postfix completions

## Features

- annotations
- auto-import
- expand macro recursively
- expand/shrink selection
- file structure
- find all references
- format string completion
- go to definition
- go to implementation
- go to type definition
- hover
- inlay hints
- join lines vs rustfmt?
- magic completions
- matching brace
- memory usage
- move item
- on enter
- on typing assists
- parent module
- related tests
- rename
- run
- semantic syntax highlighting
- show syntax tree
- status
- structural search and replace
- view Hir
- workspace symbol

### Assists

- add_explicit_type
- add_hash
- add_impl_default_members
- add_impl_missing_members
- add_lifetime_to_type
- add_turbo_fish
- apply_demorgan
- auto_import
- change_visibility
- convert_integer_literal
- convert_into_to_from
- convert_iter_for_each_to_for
- convert_to_guarded_return
- convert_tuple_struct_to_named_struct
- expand_glob_import
- extract_function
- extract_struct_from_enum_variant
- extract_type_alias
- extract_variable
- fill_match_arms
- fix_visibility
- flip_binexpr
- flip_comma
- flip_trait_bound
- generate_default_from_enum_variant
- generate_default_from_new
- generate_deref
- generate_derive
- generate_enum_as_method
- generate_enum_is_method
- generate_enum_try_into_method
- generate_from_impl_for_enum
- generate_function
- generate_getter
- generate_getter_mut
- generate_impl
- generate_is_empty_from_len
- generate_new
- generate_setter
- infer_function_return_type
- inline_function
- inline_local_variable
- introduce_named_lifetime
- invert_if
- make_raw_string
- make_usual_string
- merge_imports
- merge_match_arms
- move_arm_cond_to_match_guard
- move_bounds_to_where_clause
- move_guard_to_arm_body
- move_module_to_file
- pull_assignment_up
- qualify_path
- remove_dbg
- remove_hash
- remove_mut
- remove_unused_param
- reorder_fields
- reorder_impl
- replace_derive_with_manual_impl
  replace_for_loop_with_for_each
  replace_if_let_with_match
  replace_impl_trait_with_generic
  replace_let_with_if_let
  replace_match_with_if_let
  replace_qualified_name_with_use
  replace_string_with_char
  replace_unwrap_with_match
  split_import
  toggle_ignore
  unmerge_use
  unwrap_block
  wrap_return_type_in_result

### Diagnostics

break-outside-of-loop
inactive-code
incorrect-ident-case
macro-error
mismatched-arg-count
missing-match-arm
missing-ok-or-some-in-tail-expr
missing-pat-fields d
missing-structure-fields
missing-unsafe
no-such-field
replace-filter-map-next-with-find-map
unlinked-file
unresolved-extern-crate
unresolved-import
unresolved-macro-call
unresolved-module
unresolved-proc-macro
