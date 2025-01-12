pub mod core {
    use std::{
        fs::{self, DirEntry},
        path::Path,
    };

    #[derive(Debug)]
    pub struct Matcher {
        components: Vec<String>,
        stories: Vec<String>,
    }

    impl Matcher {
        pub fn new(components: Vec<String>, stories: Vec<String>) -> Matcher {
            Matcher {
                components,
                stories,
            }
        }

        pub fn add_component(&mut self, str: &str) {
            self.components.push(str.to_string());
        }

        pub fn add_story(&mut self, str: &str) {
            self.stories.push(str.to_string());
        }

        fn get_names(filenames: &Vec<String>) -> Vec<&str> {
            filenames
                .iter()
                .map(|c| {
                    let split_str: Vec<&str> = c.split(".").collect();
                    split_str[0]
                })
                .collect()
        }

        fn get_matches(&self) -> Vec<(String, String)> {
            let components = Matcher::get_names(&self.components);
            let stories = Matcher::get_names(&self.stories);
            let mut result: Vec<(String, String)> = vec![];

            for component in components {
                let story = stories.iter().find(|&story| **story == *component);
                if let Some(story) = story {
                    result.push((
                        format!("{component}.react.tsx"),
                        format!("{story}.stories.tsx"),
                    ));
                }
            }

            result
        }

        pub fn results(&self) {
            println!("components = {:?}", self.components);
            println!("stories = {:?}", self.stories);

            println!("total components = {:?}", self.components.len());
            println!("total stories = {:?}", self.stories.len());

            let match_results = self.get_matches();

            println!("total matches = {:?}", match_results.len());
            println!("matches = {:?}", match_results);
        }
    }

    pub fn visit_dir(dir: &Path, cb: &dyn Fn(&DirEntry)) -> Result<(), std::io::Error> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let _ = visit_dir(&path, cb);
                } else {
                    println!("file = {:?}", path);
                    cb(&entry)
                }
            }
        }

        Ok(())
    }
}

pub mod react {
    use regex::Regex;

    pub fn is_jsx(filename: &str) -> bool {
        let re = Regex::new(r"\.react\.tsx$|\.react.ts$|\.tsx$").unwrap();
        re.is_match(filename)
    }

    pub fn is_storybook(filename: &str) -> bool {
        let re = Regex::new(r"\.stories\.tsx$|\.stories.ts$").unwrap();
        re.is_match(filename)
    }
}
