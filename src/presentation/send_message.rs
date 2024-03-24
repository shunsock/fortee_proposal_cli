use colored::Colorize;

pub enum RunningStatus {
    Failed,
    Notice,
    Success,
    Warning,
}

pub fn send_message(status: RunningStatus, message: &str) {
    match status {
        RunningStatus::Failed => {
            println!("{} {}", "[failed]".red(), message);
        }
        RunningStatus::Notice => {
            println!("{} {}", "[notice]".cyan(), message);
        }
        RunningStatus::Success => {
            println!("{} {}", "[success]".green(), message);
        }
        RunningStatus::Warning => {
            println!("{} {}", "[warning]".yellow(), message);
        }
    }
}

// write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// test_check_function_showing_status_correctly
    /// Function must
    /// - show each status correctly (not raise error for these inputs)
    fn test_check_function_showing_status_correctly() {
        send_message(RunningStatus::Failed, "failed message");
        send_message(RunningStatus::Notice, "notice message");
        send_message(RunningStatus::Success, "success message");
        send_message(RunningStatus::Warning, "warning message");
    }
}
