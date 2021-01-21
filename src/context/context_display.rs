use crate::Window;

/// Kinds of `Context`'s displays.
pub enum ContextDisplay {
    /// No display.
    None,
    /// The whole screen.
    Screen,
    /// A window with name and dimensions.
    Window(Window)
}
