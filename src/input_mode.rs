#[derive(PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing,
    Saving,
    Loading,
}
