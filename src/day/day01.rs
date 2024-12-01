use std::collections::HashMap;

use anyhow::Context;

use super::Day;

pub struct Day01;

fn to_vecs(input: &str) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
    let line_count = input.lines().count();
    let mut left: Vec<i64> = Vec::with_capacity(line_count);
    let mut right: Vec<i64> = Vec::with_capacity(line_count);

    for l in input.lines() {
        let mut parts = l.split_whitespace();
        left.push(parts.next().context("Expected left number")?.parse()?);
        right.push(parts.next().context("Expected right number")?.parse()?);
    }

    Ok((left, right))
}

impl Day for Day01 {
    fn one(&self, input: &str) -> anyhow::Result<i64> {
        let (mut left, mut right) = to_vecs(input)?;

        left.sort();
        right.sort();

        Ok(left
            .into_iter()
            .zip(right)
            .map(|(left, right)| (right - left).abs())
            .sum())
    }
    fn two(&self, input: &str) -> anyhow::Result<i64> {
        let (left, right) = to_vecs(input)?;
        let mut counts: HashMap<i64, i64> = HashMap::new();

        for i in right {
            counts.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }

        Ok(left
            .into_iter()
            .map(|i| i * counts.get(&i).unwrap_or(&0))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn one() {
        let d = Day01;
        let result = d.one(&INPUT).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn two() {
        let d = Day01;
        let result = d.two(&INPUT).unwrap();
        assert_eq!(result, 31);
    }
}
