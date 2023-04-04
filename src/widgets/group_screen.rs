///
/// A widget/screen which should used a GroupScreen, should implement this trait.
///
pub trait GroupScreen {
    ///
    /// Set the index of the shown group and reload the widget.
    ///
    fn show_group(&self, group_idx: u32);

    ///
    /// Reload the widget from the data pointer.
    ///
    fn reload(&self);
}
