use std::path::PathBuf;

use stylex_shared::{
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
  StyleXTransform,
};
use swc_core::{
  common::{FileName, Mark},
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::{base::resolver, testing::test_fixture},
  },
};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
  let output = input.parent().unwrap().join("output.js");
  let output_prod = input.parent().unwrap().join("output_prod.js");

  test_fixture(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    &|tr| {
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();

      let mut config = StyleXOptionsParams {
        dev: Some(true),
        treeshake_compensation: Some(true),
        unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
        gen_conditional_classes: Some(true),
        ..StyleXOptionsParams::default()
      };

      (
        resolver(unresolved_mark, top_level_mark, false),
        StyleXTransform::new_test_force_runtime_injection_with_pass(
          tr.comments.clone(),
          PluginPass {
            cwd: None,
            filename: FileName::Real("/app/pages/Page.stylex.tsx".into()),
          },
          Some(&mut config),
        ),
      )
    },
    &input,
    &output,
    Default::default(),
  );

  test_fixture(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    &|tr| {
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();

      let mut config = StyleXOptionsParams {
        dev: Some(false),
        treeshake_compensation: Some(true),
        unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
        runtime_injection: Some(false),
        gen_conditional_classes: Some(true),
        ..StyleXOptionsParams::default()
      };

      (
        resolver(unresolved_mark, top_level_mark, false),
        StyleXTransform::new_test_with_pass(
          tr.comments.clone(),
          PluginPass {
            cwd: None,
            filename: FileName::Real("/app/pages/Page.stylex.tsx".into()),
          },
          Some(&mut config),
        ),
      )
    },
    &input,
    &output_prod,
    Default::default(),
  );
}
