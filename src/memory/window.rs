use crate::{MemorySession, MemorySessionError};

pub struct WindowMemorySession {
    turns: Vec<String>,
    window_size: usize,
}

impl WindowMemorySession{
    pub fn new(window_size: Option<usize>) -> Self {
        Self {
            turns: Vec::new(),
            window_size: window_size.unwrap_or(3),
        }
    }
}

impl MemorySession for WindowMemorySession {
    fn add_turn(&mut self, turn: String) -> Result<(), MemorySessionError> {
        self.turns.push(turn);
        Ok(())
    }

    fn retrieve_history(&mut self, _query: String) -> Result<Vec<String>, MemorySessionError> {
        let total_turns = self.turns.len();
        if total_turns == 0 {
            return Err(MemorySessionError::InternalError(format!(
                "No turns found in the conversation history."
            )))
        } else if self.window_size <= total_turns {
            let start_index = total_turns - self.window_size;
            Ok(self.turns[start_index..].to_vec())
        } else {
            Ok(self.turns.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{memory::window::WindowMemorySession, MemorySession, MemorySessionError};

    #[test]
    fn window_test() {
        let mut memory = WindowMemorySession::new(Some(2));
        memory.add_turn("Value 1".to_string()).map_err(|e| return MemorySessionError::InternalError(e.to_string())).ok();
        memory.add_turn("Value 2".to_string()).map_err(|e| MemorySessionError::InternalError(e.to_string())).ok();
        memory.add_turn("Value 3".to_string()).map_err(|e| MemorySessionError::InternalError(e.to_string())).ok();
        let result = memory.retrieve_history("sample_query".to_string()).map_err(|e| MemorySessionError::InternalError(e.to_string())).ok().unwrap();
        let target_result = ["Value 2".to_string(), "Value 3".to_string()].to_vec();
        assert_eq!(result, target_result);
    }
}
