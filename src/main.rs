use std::{cell::RefCell, env, path::Path};

use storybook_counter::{
    core::{visit_dir, Matcher},
    react::{is_jsx, is_storybook},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Missing arguments.")
    }

    let components_dir = Path::new(&args[1]);
    let stories_dir = Path::new(&args[2]);

    let components: Vec<String> = vec![];
    let stories: Vec<String> = vec![];

    let matcher = RefCell::new(Matcher::new(components, stories));

    for dir in [components_dir, stories_dir] {
        let _ = visit_dir(dir, &|entry| {
            let path = entry.path();
            let filename = path.to_str().unwrap().split("/").last().unwrap();

            if is_storybook(filename) {
                matcher.borrow_mut().add_story(filename);
            } else if is_jsx(filename) {
                matcher.borrow_mut().add_component(filename);
            }
        });
    }

    matcher.borrow().results();
}
