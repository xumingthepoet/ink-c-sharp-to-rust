use crate::compiler_conformance::api::{story::Story, story_error::StoryError};

use crate::compiler_conformance::common;

#[test]
fn tunnel_onwards_divert_override_test() -> Result<(), StoryError> {
    let json_string = common::get_ink_string("inkfiles/tunnels/tunnel-onwards-divert-override.ink");
    let mut story = Story::new(&json_string);

    assert_eq!("This is A\nNow in B.\n", story.continue_maximally());

    Ok(())
}
