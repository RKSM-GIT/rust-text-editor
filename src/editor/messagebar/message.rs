use std::time::{Duration, Instant};

const EXPIRE_TIME: Duration = Duration::new(5, 0);

#[derive(Debug)]
pub struct Message {
    pub text: String,
    pub time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self { 
            text: Default::default(), 
            time: Instant::now(), 
        }
    }
}

impl Message {
    pub fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > EXPIRE_TIME
    }
}