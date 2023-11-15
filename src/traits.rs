use include_dir::{include_dir, Dir};

static INPUT_DIR: Dir = include_dir!("input");

pub trait Solves {

    const DAY: u8;

    fn input(&self) -> &'static str {
        let file: String;
        if Self::DAY < 10 {
            file = format!("0{}", Self::DAY);
        } else {
            file = format!("{}", Self::DAY);
        }
        INPUT_DIR.get_file(file.as_str())
            .unwrap()
            .contents_utf8()
            .unwrap()
    }

    fn part_one(&self, _input: &str) -> String {
        String::from("Not implemented")
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("Not implemented")
    }

}