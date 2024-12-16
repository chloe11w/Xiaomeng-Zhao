#[cfg(test)]
mod tests {
    use crate::analysis::normalize_breed;
    use crate::utils::{parse_age, extract_month_and_season};

    #[test]
    fn test_normalize_breed() {
        assert_eq!(normalize_breed("American Pit Bull Mix / Pit Bull Mix"), "Pit Bull");
        assert_eq!(normalize_breed("Mixed/Other"), "Mixed");
        assert_eq!(normalize_breed("MIXED BREED"), "Mixed");
        assert_eq!(normalize_breed("German Shepherd"), "German");
        assert_eq!(normalize_breed("Unknown"), "Unknown");
    }

    #[test]
    fn test_parse_age() {
        assert_eq!(parse_age(&Some("4Y".to_string())), Some(4.0));
        assert_eq!(parse_age(&Some("5".to_string())), Some(5.0));
        assert_eq!(parse_age(&Some("".to_string())), None);
        assert_eq!(parse_age(&None), None);
    }

    #[test]
    fn test_extract_month_and_season() {
        assert_eq!(
            extract_month_and_season(&Some("January 15 2022".to_string())),
            Some((1, 1)) // 1月 
        );
        assert_eq!(
            extract_month_and_season(&Some("July 04 2022".to_string())),
            Some((7, 3)) // 7月 
        );
        assert_eq!(
            extract_month_and_season(&Some("October 20 2022".to_string())),
            Some((10, 4)) // 10月 
        );
        assert_eq!(
            extract_month_and_season(&Some("Invalid Date".to_string())),
            None
        );
    }
}
