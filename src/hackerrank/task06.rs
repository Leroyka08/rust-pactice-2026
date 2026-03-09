#[allow(dead_code)]
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    match v1 > v2 && (x2 - x1) % (v1 - v2) == 0 {
        true => String::from("YES"),
        false => String::from("NO"),
    }
}

#[cfg(test)]
mod tests {
    use super::kangaroo;

    #[test]
    fn test_kangaroo_meets() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_never_meets_speed() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_kangaroo_never_meets_distance() {
        assert_eq!(kangaroo(43, 2, 70, 2), "NO");
    }
}