/// The different actions that can occur in the VM.
#[derive(Debug)]
pub enum Action {
    /// Stops the VM allowing to resume its execution.
    Halt,

    /// Stops the VM because of an error.
    Panic(&'static str),
}

impl Action {
    // GETTERS ----------------------------------------------------------------

    pub fn is_halt(&self) -> bool {
        matches!(self, Action::Halt)
    }

    pub fn is_panic(&self) -> bool {
        matches!(self, Action::Panic(_))
    }

    // METHODS ----------------------------------------------------------------

    pub fn unwrap_panic(self) -> &'static str {
        match self {
            Action::Panic(v) => v,
            _ => unreachable!(),
        }
    }
}
