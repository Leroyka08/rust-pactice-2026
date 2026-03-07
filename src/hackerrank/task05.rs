#[allow(dead_code)]
fn calculate_fruits_in_range(s: i32, t: i32, tree_pos: i32, fruits: &[i32]) -> usize {
    fruits
        .iter()
        .filter(|&&distance| (s..=t).contains(&(tree_pos + distance)))
        .count()
}

#[allow(dead_code)]
pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apples_count = calculate_fruits_in_range(s, t, a, apples);
    let oranges_count = calculate_fruits_in_range(s, t, b, oranges);

    println!("{}", apples_count);
    println!("{}", oranges_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fruits_in_range() {
        let s = 7;
        let t = 11;
        
        let a = 5;
        let apples = [-2, 2, 1];
        assert_eq!(calculate_fruits_in_range(s, t, a, &apples), 1);

        let b = 15;
        let oranges = [5, -6];
        assert_eq!(calculate_fruits_in_range(s, t, b, &oranges), 1);
    }

    #[test]
    fn test_execution() {
        count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
    }
}