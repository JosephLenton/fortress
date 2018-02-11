use LLRKey;

/// An event on the input system.
pub enum LLREvent {
    /// When the application quits.
    /// This could be the process is told to quit, or the window has shut, or
    /// something else.
    ///
    /// Whatever the cause. If you get this event the LLR implementation
    /// considers it's self to be quitting.
    Quit,

    /// Happens when the LLR is resized.
    /// If you want the new size then you'll have to query the LLR for it's
    /// current size.
    Resize,

    /// When user input has occurred.
    /// This is the key that was pressed.
    KeyPress(LLRKey),
}
