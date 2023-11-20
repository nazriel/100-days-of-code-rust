fn round_price(input: f32) -> f32 {
    (input * 100.0).ceil() / 100.0
}

pub fn calculate_tip(total: f32, percentage: u8, people: u32) -> Option<f32> {
    match (total, percentage, people) {
        (_, _, 0) => None,
        (_, 0, _) => Some(round_price(total / people as f32)),
        _ if total == 0.0 => None,
        _ => {
            let tip = total * (percentage as f32 / 100.0);
            let total_with_tip = total + tip;
            Some(round_price(total_with_tip / people as f32))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::calculate_tip;

    #[test]
    fn test_tip_zeros() {
        assert_eq!(calculate_tip(0.0, 0, 0), None);
        assert_eq!(calculate_tip(0.0, 10, 10), None);
        assert_eq!(calculate_tip(10.0, 10, 0), None);
    }

    #[test]
    fn test_no_tip() {
        assert_eq!(calculate_tip(100.0, 0, 1), Some(100.0));
    }

    #[test]
    fn test_proper_rounding() {
        assert_eq!(calculate_tip(100.0, 0, 3), Some(33.34));
        assert_eq!(calculate_tip(100.0, 0, 4), Some(25.0));
        assert_eq!(calculate_tip(100.0, 0, 1), Some(100.0));
    }

    #[test]
    fn test_success() {
        assert_eq!(calculate_tip(50.0, 10, 2), Some(27.5));
    }
}
