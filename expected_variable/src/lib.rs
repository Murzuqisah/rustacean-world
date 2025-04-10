extern crate case;

use case::CaseExt;

pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[len1][len2]
}

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    let compare_lower = compare.to_lowercase();
    let expected_lower = expected.to_lowercase();

    if (compare_lower.to_ascii_lowercase() == compare_lower
        || compare_lower.to_camel_lowercase() == compare_lower)
        && !compare_lower.contains("-")
        && !compare_lower.contains(" ")
    {
        let distance = edit_distance(&compare_lower, &expected_lower) as i64;

        if distance == 0 {
            return Some("100%".to_string());
        }

        let percentage = 100 - (distance * 100 / expected.len() as i64);
        if percentage >= 50 {
            let mut res = percentage.to_string();
            res.push_str("%");
            return Some(res);
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert_eq!(expected_variable("on_point", "on_point"), Some("100%".to_string()));
        assert_eq!(expected_variable("soClose", "so_close"), Some("88%".to_string()));
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(expected_variable("On_Point", "on_point"), Some("100%".to_string()));
        assert_eq!(expected_variable("soClose", "so_close"), Some("88%".to_string()));
    }

    #[test]
    fn test_not_camel_or_snake_case() {
        assert_eq!(expected_variable("notkebab-case", "something"), None);
        assert_eq!(expected_variable("space case", "another"), None);
    }

    #[test]
    fn test_low_alikeness() {
        assert_eq!(expected_variable("something", "something_completely_different"), None);
    }

    #[test]
    fn test_high_alikeness() {
        assert_eq!(expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch"), Some("67%".to_string()));
    }

    #[test]
    fn test_empty_expected() {
        assert_eq!(expected_variable("value", ""), None);
        assert_eq!(expected_variable("", ""), Some("100%".to_string()));
    }
}