use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::{glib, subclass::widget::*, traits::WidgetExt, FlowBox, Label, LayoutManager, Orientation, Widget};
mod last_competitions;
mod quick_settings;
use super::navbar::NavBar;
use crate::MainNavBarCategory;
use last_competitions::LastCompetitionsWidget;
use quick_settings::QuickSettingsWidget;
use std::cell::RefCell;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct HomeScreen {
        flow_box: FlowBox,
        title: Label,
        pub quick_settings_widget: RefCell<Option<QuickSettingsWidget>>,
    }

    impl Default for HomeScreen {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HomeScreen {
        const NAME: &'static str = "HomeScreen";
        type Type = super::HomeScreen;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("home_screen");
        }
    }

    impl ObjectImpl for HomeScreen {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let last_competitions = LastCompetitionsWidget::new();
            self.flow_box.insert(&last_competitions, -1);

            let quick_settings = QuickSettingsWidget::new();
            self.flow_box.insert(&quick_settings, -1);
            *self.quick_settings_widget.borrow_mut() = Some(quick_settings);

            self.title.set_parent(&*obj);
            self.flow_box.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.flow_box.unparent();
            self.title.unparent();
        }
    }

    impl WidgetImpl for HomeScreen {}

    impl HomeScreen {
        fn new() -> Self {
            Self {
                flow_box: FlowBox::builder()
                    .max_children_per_line(2)
                    .min_children_per_line(2)
                    .orientation(gtk4::Orientation::Horizontal)
                    .selection_mode(gtk4::SelectionMode::None)
                    .homogeneous(true)
                    .build(),
                title: Label::builder().label("ISRAT").css_classes(["headline"]).build(),
                quick_settings_widget: RefCell::default(),
            }
        }
    }
}

glib::wrapper! {
    pub struct HomeScreen(ObjectSubclass<inner::HomeScreen>)
        @extends Widget;
}

impl HomeScreen {
    pub fn new(nav_bar: &NavBar<MainNavBarCategory>) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.property::<LayoutManager>("layout_manager")
            .set_property("orientation", Orientation::Vertical);
        obj.set_hexpand(true);
        (*obj.imp().quick_settings_widget.borrow())
            .as_ref()
            .unwrap()
            .connect_signals(nav_bar.clone());
        obj
    }
}
