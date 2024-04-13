use std::env;

pub(crate) fn show_home_dir() -> String {
    env::var("HOME").expect("HOME directory not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(ci_env, ignore)]
    fn test_success() {
        let home_dir: String = show_home_dir();
        assert!(!home_dir.is_empty());
    }
}
