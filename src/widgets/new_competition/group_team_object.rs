use gtk4::{
    glib::{self, Object, ParamSpec, Properties, Value},
    prelude::*,
    subclass::prelude::*,
};
use std::cell::RefCell;

mod inner {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::GroupTeamObject)]
    pub struct GroupTeamObject {
        #[property(get, set)]
        pub group_name: RefCell<String>,
        #[property(get, set)]
        pub team_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GroupTeamObject {
        const NAME: &'static str = "GroupTeamObject";
        type Type = super::GroupTeamObject;
    }

    impl ObjectImpl for GroupTeamObject {
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
    pub struct GroupTeamObject(ObjectSubclass<inner::GroupTeamObject>);
}

impl GroupTeamObject {
    pub fn new(group_name: String, team_name: String) -> Self {
        Object::builder()
            .property("group-name", group_name)
            .property("team-name", team_name)
            .build()
    }
}
