use std::fs;

use anyhow::{Context, Result};

use crate::Part;

mod day01;
mod day02;

trait Day {
    fn one(&self, input: &str) -> Result<i64>;
    fn two(&self, input: &str) -> Result<i64>;
}

pub struct Days {
    list: Vec<Box<dyn Day>>,
}

impl Days {
    pub fn new() -> Self {
        Self {
            list: vec![Box::new(day01::Day01), Box::new(day02::Day02)],
        }
    }

    pub fn run(&self, day: u8, part: Part) -> Result<()> {
        assert!(day >= 1);
        assert!(day <= 24);

        let d = self
            .list
            .get((day - 1) as usize)
            .context("Day does not exist")?;

        let input_path = std::env::current_dir()?
            .to_path_buf()
            .join("input/")
            .join(format!("day{:02}.txt", day));

        let input = fs::read_to_string(&input_path).with_context(|| {
            format!(
                "Failed to load input file for day {} from {:?}",
                day, &input_path
            )
        })?;

        let res = match part {
            Part::One => d.one(&input)?,
            Part::Two => d.two(&input)?,
        };

        print!("Day: {}, Part: {}, Result: {}", day, part, res);
        Ok(())
    }
}
