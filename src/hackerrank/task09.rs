#[allow(dead_code)]
pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6]; 

    for &bird_id in arr {
        counts[bird_id as usize] += 1;
    }

    counts
        .iter()
        .enumerate()
        .skip(1) 
        .max_by(|(id_a, count_a), (id_b, count_b)| {
            count_a.cmp(count_b).then_with(|| id_b.cmp(id_a))
        })
        .map(|(id, _)| id as i32)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::migratory_birds;

    #[test]
    fn test_most_frequent() {
        let birds = [1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&birds), 4);
    }

    #[test]
    fn test_tie_smallest_id() {
        let birds = [1, 2, 2, 1, 3];
        assert_eq!(migratory_birds(&birds), 1);
    }
}