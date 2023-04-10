use chrono::Timelike;
use gdk4::{prelude::*, subclass::prelude::*};
use gtk4::{glib, subclass::widget::*, traits::WidgetExt, Adjustment, LayoutManager, Orientation, SpinButton, Widget};

mod inner {
    use gtk4::{traits::EditableExt, Inhibit};

    use super::*;

    #[derive(Debug)]
    pub struct TimeSelector {
        /// The SpinButtons used to select hours and minutes, spin buttons are in this order.
        spin_buttons: [SpinButton; 2],
    }

    impl Default for TimeSelector {
        fn default() -> Self {
            Self::new()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TimeSelector {
        const NAME: &'static str = "TimeSelector";
        type Type = super::TimeSelector;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("time_selector");
        }
    }

    impl ObjectImpl for TimeSelector {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.property::<LayoutManager>("layout_manager")
                .set_property("orientation", Orientation::Horizontal);

            for spin_button in self.spin_buttons.iter() {
                spin_button.set_parent(&*obj);
                // show leading zeros
                spin_button.connect_output(|spin_button| {
                    spin_button.set_text(&format!("{:0>2}", spin_button.value_as_int()));
                    Inhibit(true)
                });
            }

            self.spin_buttons[0].connect_value_changed(|spin_button| {
                println!("[0]: float: {}", spin_button.value());
                println!("[0]: int: {}", spin_button.value_as_int());
            });

            self.spin_buttons[1].connect_value_changed(|spin_button| {
                println!("[1]: float: {}", spin_button.value());
                println!("[1]: int: {}", spin_button.value_as_int());
            });
        }

        fn dispose(&self) {
            for spin_button in self.spin_buttons.iter() {
                spin_button.unparent();
            }
        }
    }

    impl WidgetImpl for TimeSelector {}

    impl TimeSelector {
        fn new() -> Self {
            let hours_adjustment = Adjustment::new(0.0, 0.0, 23.0, 1.0, 0.0, 0.0);
            let minutes_adjustment = Adjustment::new(0.0, 0.0, 59.0, 1.0, 0.0, 0.0);
            let hours = SpinButton::builder()
                .adjustment(&hours_adjustment)
                .climb_rate(1.0)
                .digits(0)
                .orientation(Orientation::Vertical)
                .update_policy(gtk4::SpinButtonUpdatePolicy::IfValid)
                .build();
            let minutes = SpinButton::builder()
                .adjustment(&minutes_adjustment)
                .climb_rate(1.0)
                .digits(0)
                .orientation(Orientation::Vertical)
                .update_policy(gtk4::SpinButtonUpdatePolicy::IfValid)
                .build();
            Self {
                spin_buttons: [hours, minutes],
            }
        }

        pub fn hours(&self) -> u32 {
            self.spin_buttons[0].value_as_int() as u32
        }

        pub fn minutes(&self) -> u32 {
            self.spin_buttons[1].value_as_int() as u32
        }

        pub fn set_defaults(&self, hours_default: u32, minutes_default: u32) {
            self.spin_buttons[0].set_value(hours_default as f64);
            self.spin_buttons[1].set_value(minutes_default as f64);
        }
    }
}

glib::wrapper! {
    pub struct TimeSelector(ObjectSubclass<inner::TimeSelector>)
        @extends Widget;
}

impl TimeSelector {
    ///
    /// Creates a new TimeSelector. The default values will be inferred from the current time.
    ///
    pub fn new() -> Self {
        let now = chrono::Local::now();
        Self::with_defaults(now.hour(), now.minute())
    }

    ///
    /// Creates a new TimeSelector with default values for the hours and minutes selectors.
    ///
    pub fn with_defaults(hours_default: u32, minutes_default: u32) -> Self {
        assert!(
            hours_default < 24,
            "Please specify a default value for the hours in the interval [0, 23]!"
        );
        assert!(
            minutes_default < 60,
            "Please specify a default value for the minutes in the interval [0, 59]!"
        );

        let obj = glib::Object::new::<Self>();
        obj.imp().set_defaults(hours_default, minutes_default);
        obj
    }

    ///
    /// Returns the currently selected hours.
    /// The return value will always be in the interval [0, 23].
    ///
    pub fn hours(&self) -> u32 {
        self.imp().hours()
    }

    ///
    /// Returns the currently selected minutes.
    /// The return value will always be in the interval [0, 59].
    ///
    pub fn minutes(&self) -> u32 {
        self.imp().minutes()
    }

    ///
    /// Returns both hours and minutes, in this order.
    ///
    pub fn time(&self) -> [u32; 2] {
        [self.hours(), self.minutes()]
    }
}
