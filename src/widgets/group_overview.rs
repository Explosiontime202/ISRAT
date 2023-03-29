use super::navbar::NavBar;
use crate::CompetitionPtr;
use crate::data::Competition;
use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::{
    glib, subclass::widget::*, traits::WidgetExt, FlowBox, Label, LayoutManager, Orientation,
    Widget,
};
use std::cell::RefCell;
use std::rc::Rc;

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct GroupOverviewScreen {
        flow_box: RefCell<Option<FlowBox>>,
        title: RefCell<Option<Label>>,
        // pub quick_settings_widget: RefCell<Option<QuickSettingsWidget>>,
        data: RefCell<Option<CompetitionPtr>>,
    }

    impl Default for GroupOverviewScreen {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GroupOverviewScreen {
        const NAME: &'static str = "GroupOverviewScreen";
        type Type = super::GroupOverviewScreen;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("group_overview");
        }
    }

    impl ObjectImpl for GroupOverviewScreen {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            // let last_competitions = LastCompetitionsWidget::new();
            // self.flow_box.insert(&last_competitions, -1);

            // let quick_settings = QuickSettingsWidget::new();
            // self.flow_box.insert(&quick_settings, -1);
            // *self.quick_settings_widget.borrow_mut() = Some(quick_settings);
        }

        fn dispose(&self) {
        }
    }

    impl WidgetImpl for GroupOverviewScreen {
        fn show(&self) {
            let title = &*self.title.borrow();
            let flow_box = &*self.flow_box.borrow();
            debug_assert!(title.is_none());
            debug_assert!(flow_box.is_none());
            self.init_children();
        }

        fn hide(&self) {
            let title = &*self.title.borrow();
            let flow_box = &*self.flow_box.borrow();
            debug_assert!(title.is_some());
            debug_assert!(flow_box.is_some());

            title.as_ref().unwrap().unparent();
            flow_box.as_ref().unwrap().unparent();
        }
    }

    impl GroupOverviewScreen {
        fn new() -> Self {
            Self {
                flow_box: RefCell::default(),
                title: RefCell::default(),
                data: RefCell::default(),
            }
        }

        fn init_children(&self) {
            let flow_box = FlowBox::builder()
                .max_children_per_line(2)
                .min_children_per_line(2)
                .orientation(gtk4::Orientation::Horizontal)
                .selection_mode(gtk4::SelectionMode::None)
                .homogeneous(true)
                .build();

            let title = Label::builder().label("").css_classes(["headline"]).build();

            flow_box.set_parent(&*self.obj());
            title.set_parent(&*self.obj());

            *self.flow_box.borrow_mut() = Some(flow_box);
            *self.title.borrow_mut() = Some(title);
        }

        pub fn set_data(&self, data: CompetitionPtr) {
            *self.data.borrow_mut() = Some(data);
        }
    }
}

glib::wrapper! {
    pub struct GroupOverviewScreen(ObjectSubclass<inner::GroupOverviewScreen>)
        @extends Widget;
}

impl GroupOverviewScreen {
    pub fn new(competition: CompetitionPtr) -> Self {
        let obj = glib::Object::new::<Self>();
        obj.property::<LayoutManager>("layout_manager")
            .set_property("orientation", Orientation::Vertical);
        obj.set_hexpand(true);
        obj.imp().set_data(competition);
        obj
    }
}
