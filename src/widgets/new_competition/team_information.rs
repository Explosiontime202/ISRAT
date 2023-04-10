/*use gdk4::subclass::prelude::*;
use gtk4::{glib, subclass::widget::*, traits::*, Box as GtkBox, Button, Widget};

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct BaseInformationScreen {}

    impl Default for BaseInformationScreen {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BaseInformationScreen {
        const NAME: &'static str = "BaseInformationScreen";
        type Type = super::BaseInformationScreen;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("base_information");
        }
    }

    impl ObjectImpl for BaseInformationScreen {}
    impl WidgetImpl for BaseInformationScreen {}

    impl BaseInformationScreen {
        pub fn new() -> Self {
            Self {}
        }
    }
}

glib::wrapper! {
    pub struct BaseInformationScreen(ObjectSubclass<inner::BaseInformationScreen>)
        @extends Widget;
}

impl BaseInformationScreen {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }
}
*/
