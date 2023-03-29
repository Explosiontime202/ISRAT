use gdk4::{
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, MemoryInputStream},
    glib::Bytes,
};
use gtk4::Image;

///
/// Creates a gtk image from bytes. Designed for images included in the binary.
///
pub fn img_from_bytes(bytes: &'static [u8]) -> Image {
    let bytes = Bytes::from(bytes);
    let stream = MemoryInputStream::from_bytes(&bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    let image = Image::from_pixbuf(Some(&pixbuf));
    image.set_icon_size(gtk4::IconSize::Large);

    image
}
