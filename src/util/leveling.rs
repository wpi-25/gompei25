

fn get_level_number(xp: u32) -> u32 {
    let xp = xp as f64;
    let mut level = (xp / 50f64).powf(1f64/3f64).floor();

    if level < 0f64 {
        level = 0f64;
    }

    level as u32
}

fn get_level_cost(level: u64) -> u64 {
    (level.pow(3) * 50).into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_xp() {
        assert_eq!(super::get_level_number(0), 0)
    }
    #[test]
    fn level_3() {
        assert_eq!(super::get_level_number(2757), 3)
    }
    #[test]
    fn level_14() {
        assert_eq!(get_level_number(149818), 14)
    }

    #[test]
    fn cost_level_3() {
        assert_eq!(get_level_cost(3), 1350)
    }

    #[test]
    fn cost_level_14() {
        assert_eq!(get_level_cost(14), 137200)
    }
}
