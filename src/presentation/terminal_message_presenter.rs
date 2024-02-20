use colored::Colorize;

pub struct ConsoleMessager {
    pub(crate) message: String,
    pub(crate) message_type: MessageType,
}

pub enum MessageType {
    Failed,
    Warning,
    Success,
}

impl ConsoleMessager {
    pub fn new(message: String, message_type: MessageType) -> ConsoleMessager {
        ConsoleMessager {
            message,
            message_type,
        }
    }

    pub fn show_message(&self) {
        match self.message_type {
            MessageType::Failed => {
                println!("{} {}", "[failed]".red(), self.message);
            }
            MessageType::Success => {
                println!("{} {}", "[success]".green(), self.message);
            }
            MessageType::Warning => {
                println!("{} {}", "[warning]".yellow(), self.message);
            }
        }
    }

    pub fn supply_message(&self) -> String {
        match self.message_type {
            MessageType::Failed => {
                format!("{} {}", "[failed]".red(), self.message)
            }
            MessageType::Success => {
                format!("{} {}", "[success]".green(), self.message)
            }
            MessageType::Warning => {
                format!("{} {}", "[warning]".yellow(), self.message)
            }
        }
    }
}
