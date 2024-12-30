use super::Day;

pub struct Day02;

fn is_valid_creasing(s: &[i32]) -> bool {
    let mut it = s.windows(2);
    let Some(first) = it.next() else { return false };
    if first[0] < first[1] {
        return !it.any(|v| v[0] > v[1]);
    } else if first[0] > first[1] {
        return !it.any(|v| v[0] < v[1]);
    }
    false
}

fn is_valid_diff(s: &[i32]) -> bool {
    for w in s.windows(2) {
        let abs = (w[0] - w[1]).abs();
        if abs > 3 || abs < 1 { return false}
    }
    true
}

impl Day for Day02 {
    fn one(&self, input: &str) -> anyhow::Result<i64> {
        let mut safe = 0;
        for line in input.lines() {
            let digits = line.split_whitespace()
                .map(|d| d.parse::<i32>())
                .collect::<Result<Vec<_>,_>>()?;
            if is_valid_creasing(&digits) && is_valid_diff(&digits) { 
                safe += 1
            }
        }
        Ok(safe)
    }

    fn two(&self, input: &str) -> anyhow::Result<i64> {
        let mut safe = 0;
        for line in input.lines() {
            let digits = line.split_whitespace()
                .map(|d| d.parse::<i32>())
                .collect::<Result<Vec<_>,_>>()?;

            let mut mutations: Vec<Vec<i32>> = Vec::new();
            for i in 0..digits.len() {
                let mut v = digits.clone();
                v.remove(i);
                mutations.push(v);
            }
            for m in mutations {
                if is_valid_creasing(&m) && is_valid_diff(&m) { 
                    safe += 1;
                    break;
                }
            }
        }
        Ok(safe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn one() {
        let d = Day02;
        let result = d.one(&INPUT).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn two() {
        let d = Day02;
        let result = d.two(&INPUT).unwrap();
        assert_eq!(result, 4);
    }
}
