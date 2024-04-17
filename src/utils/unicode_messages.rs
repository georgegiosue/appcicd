use std::fmt;

pub enum UMessage<'a> {
    SUCCESS(&'a str),
    INFO(&'a str),
    WARNING(&'a str),
    ERROR(&'a str),
    DEPLOY(&'a str),
    PWD(&'a str),
    INPUT(&'a str),
    ROLLBACK(&'a str),
}

impl<'a> fmt::Display for UMessage<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UMessage::SUCCESS(msg) => write!(f, "✅ {}", msg),
            UMessage::INFO(msg) => write!(f, "🔍 {}", msg),
            UMessage::WARNING(msg) => write!(f, "⚠️ {}", msg),
            UMessage::ERROR(msg) => write!(f, "❌ {}", msg),
            UMessage::DEPLOY(msg) => write!(f, "🚀 {}", msg),
            UMessage::PWD(msg) => write!(f, "🔑 {}", msg),
            UMessage::INPUT(msg) => write!(f, "📝 {}", msg),
            UMessage::ROLLBACK(msg) => write!(f, "↩️ {}", msg),
        }
    }
}
