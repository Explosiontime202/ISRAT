use gtk4::{
    glib::{self, Object, ParamSpec, Properties, Value},
    prelude::*,
    subclass::prelude::*,
    EntryBuffer,
};
use std::cell::RefCell;

mod inner {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::TeamRegionObject)]
    pub struct TeamRegionObject {
        #[property(get, set)]
        pub team: RefCell<EntryBuffer>,
        #[property(get, set)]
        pub region: RefCell<EntryBuffer>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TeamRegionObject {
        const NAME: &'static str = "TeamRegionObject";
        type Type = super::TeamRegionObject;
    }

    impl ObjectImpl for TeamRegionObject {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }
}

glib::wrapper! {
    pub struct TeamRegionObject(ObjectSubclass<inner::TeamRegionObject>);
}

impl TeamRegionObject {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_default(team: EntryBuffer, region: EntryBuffer) -> Self {
        Object::builder().property("team", team).property("region", region).build()
    }
}

impl Default for TeamRegionObject {
    fn default() -> Self {
        Self::new()
    }
}
