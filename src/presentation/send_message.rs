use colored::Colorize;

pub enum RunningStatus {
    Failed,
    Notice,
    Success,
}

pub fn send_message_to_console(status: RunningStatus, message: &str) {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    /// test_check_function_showing_status_correctly
    /// Function must
    /// - show each status correctly (not raise error for these inputs)
    fn test_check_function_showing_status_correctly() {
        send_message_to_console(RunningStatus::Failed, "failed message");
        send_message_to_console(RunningStatus::Notice, "notice message");
        send_message_to_console(RunningStatus::Success, "success message");
    }
}
