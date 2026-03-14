#[allow(dead_code)]
pub fn day_of_the_programmer(year: i32) -> String {
    if year == 1918 {
        return String::from("26.09.1918");
    }

    let is_leap = if year < 1918 {
        year % 4 == 0
    } else {
        (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
    };

    if is_leap {
        format!("12.09.{}", year)
    } else {
        format!("13.09.{}", year)
    }
}

#[cfg(test)]
mod tests {
    use super::day_of_the_programmer;

    #[test]
    fn test_julian_leap() {
        assert_eq!(day_of_the_programmer(1800), "12.09.1800");
    }

    #[test]
    fn test_gregorian_leap() {
        assert_eq!(day_of_the_programmer(2000), "12.09.2000");
    }

    #[test]
    fn test_transition_year() {
        assert_eq!(day_of_the_programmer(1918), "26.09.1918");
    }

    #[test]
    fn test_regular_year() {
        assert_eq!(day_of_the_programmer(2017), "13.09.2017");
    }
}