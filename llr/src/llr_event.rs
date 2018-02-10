
use LLRKey;

/// An event on the input system.
pub enum LLREvent {
    Quit,
    Resize,
    KeyPress(LLRKey),
}

