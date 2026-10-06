pub mod features;
pub mod runner;
pub mod sys;

pub enum Step {
    Done(String),
    Idle(String),
    Failed(String),
}

impl Step {
    pub fn is_done(&self) -> bool {
        matches!(self, Step::Done(_))
    }

    pub fn note(&self) -> &str {
        match self {
            Step::Done(n) | Step::Idle(n) | Step::Failed(n) => n,
        }
    }
}
