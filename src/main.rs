use std::{env, path::Path, sync::Mutex};

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

    let matcher = Mutex::new(Matcher::new(components, stories));

    for dir in [components_dir, stories_dir] {
        let _ = visit_dir(dir, &|entry| {
            let path = entry.path();
            let filename = path.to_str().unwrap().split("/").last().unwrap();
            let mut m = matcher.lock().unwrap();

            // Need to check storybook first as the logic use `ends_with`
            // This need to be improved by using regexp instead to fix the order constraint.
            if is_storybook(filename) {
                m.add_story(filename);
            } else if is_jsx(filename) {
                m.add_component(filename);
            }
        });
    }

    matcher.lock().unwrap().results();
}
