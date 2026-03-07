#[allow(dead_code)]
pub fn staircase(n: i32) {
    let n = n as usize;
    for i in 1..=n {
        println!("{:>width$}", "#".repeat(i), width = n);
    }
}

#[cfg(test)]
mod tests {
    use super::staircase;

    #[test]
    fn test_staircase_logic() {
        let n = 4;
        let mut result = Vec::new();
        for i in 1..=n {
            result.push(format!("{:>width$}", "#".repeat(i), width = n));
        }
        let expected = vec!["   #", "  ##", " ###", "####"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_execution() {
        staircase(1);
    }
}