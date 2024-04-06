use chrono::Duration;

pub fn duration_to_str(duration: Duration) -> String {
    if duration == Duration::zero() {
        return "0m".to_string();
    }

    let weeks = duration.num_weeks();
    let days = duration.num_days() % 7;
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;

    let mut output = String::new();
    if weeks != 0 {
        output.push_str(&format!("{}w ", weeks));
    }
    if days != 0 {
        output.push_str(&format!("{}d ", days));
    }
    if hours != 0 {
        output.push_str(&format!("{}h ", hours));
    }
    if minutes != 0 {
        output.push_str(&format!("{}m", minutes));
    }

    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_duration_to_str() {
        assert_eq!(
            duration_to_str(
                Duration::weeks(1) + Duration::days(2) + Duration::hours(3) + Duration::minutes(4)
            ),
            "1w 2d 3h 4m"
        );
        assert_eq!(
            duration_to_str(Duration::weeks(1) + Duration::days(2) + Duration::hours(3)),
            "1w 2d 3h"
        );
        assert_eq!(
            duration_to_str(Duration::weeks(1) + Duration::days(2)),
            "1w 2d"
        );
        assert_eq!(duration_to_str(Duration::weeks(1)), "1w");
        assert_eq!(duration_to_str(Duration::days(2)), "2d");
        assert_eq!(duration_to_str(Duration::hours(3)), "3h");
        assert_eq!(duration_to_str(Duration::minutes(4)), "4m");
        assert_eq!(
            duration_to_str(Duration::days(1) - Duration::hours(1)),
            "23h"
        );
        assert_eq!(duration_to_str(Duration::zero()), "0m");
    }
}
