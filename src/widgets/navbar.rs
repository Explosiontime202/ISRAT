use gdk4::glib::clone;
use gdk4::glib::once_cell::sync::Lazy;
use gdk4::glib::{
    clone::Downgrade,
    subclass::{
        register_type,
        types::{self, FromObject},
        ObjectImplRef, ObjectImplWeakRef, TypeData,
    },
    Type,
};
use gdk4::prelude::*;
use gdk4::subclass::prelude::*;
use gtk4::{glib, subclass::widget::*, traits::*, Box as GtkBox, Button, Label, Separator, Stack, Widget};
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Once;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
};

mod inner {
    use std::mem;

    use super::*;

    #[derive(Debug)]
    pub struct NavBar<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> {
        /// Stores the children and only showing one at a time.
        stack: Stack,
        /// Different separators between the main widgets.
        /// [0] = separator between `navigation_box` and `stack`
        separators: [Separator; 1],
        /// Contains the buttons of the categories as well as the separators between the categories (not to be confused with `separators`)
        navigation_box: GtkBox,
        /// Stores the information about each category, i.e. what buttons and if a separator to the previous category exists
        buttons: RefCell<BTreeMap<CategoryT, CategoryEntry>>,
        /// Stores which button in each category is selected, it is not necessary for a category to have a selected button
        selected_buttons: Rc<RefCell<HashMap<CategoryT, Button>>>,
    }

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> Default for NavBar<CategoryT> {
        fn default() -> Self {
            Self::new()
        }
    }

    unsafe impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> ObjectSubclassType for NavBar<CategoryT> {
        #[inline]
        fn type_data() -> ::std::ptr::NonNull<TypeData> {
            static mut DATA: TypeData = types::INIT_TYPE_DATA;
            unsafe { ::std::ptr::NonNull::from(&mut DATA) }
        }

        #[inline]
        fn type_() -> Type {
            // Make sure to register the type for every generic. CAUTION: this differs glib::object_subclass proc macro.
            static mut ONCE_MAP: Lazy<HashMap<&'static str, Once>> = Lazy::new(|| HashMap::new());

            unsafe {
                if !ONCE_MAP.contains_key(CategoryT::NAME) {
                    ONCE_MAP.insert(CategoryT::NAME, Once::new());
                }
                ONCE_MAP[CategoryT::NAME].call_once(|| {
                    register_type::<Self>();
                })
            }
            unsafe {
                let data = Self::type_data();
                let type_ = data.as_ref().type_();

                type_
            }
        }
    }

    #[doc(hidden)]
    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> FromObject for NavBar<CategoryT> {
        type FromObjectType = <Self as ObjectSubclass>::Type;
        #[inline]
        fn from_object(obj: &Self::FromObjectType) -> &Self {
            <Self as ObjectSubclassExt>::from_obj(obj)
        }
    }

    #[doc(hidden)]
    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> Downgrade for NavBar<CategoryT> {
        type Weak = ObjectImplWeakRef<NavBar<CategoryT>>;

        #[inline]
        fn downgrade(&self) -> Self::Weak {
            let ref_counted = ObjectSubclassExt::ref_counted(self);
            Downgrade::downgrade(&ref_counted)
        }
    }

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> NavBar<CategoryT> {
        #[inline]
        pub fn downgrade(&self) -> <Self as Downgrade>::Weak {
            Downgrade::downgrade(self)
        }
    }

    #[doc(hidden)]
    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> ::std::borrow::ToOwned for NavBar<CategoryT> {
        type Owned = ObjectImplRef<NavBar<CategoryT>>;

        #[inline]
        fn to_owned(&self) -> Self::Owned {
            ObjectSubclassExt::ref_counted(self)
        }
    }

    #[doc(hidden)]
    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> ::std::borrow::Borrow<NavBar<CategoryT>> for ObjectImplRef<NavBar<CategoryT>> {
        #[inline]
        fn borrow(&self) -> &NavBar<CategoryT> {
            self
        }
    }

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> ObjectSubclass for NavBar<CategoryT> {
        // NavBar<CategoryT::NAME>
        // const NAME: &'static str = concatcp!("NavBar<", , ">");
        const NAME: &'static str = CategoryT::NAV_BAR_NAME;
        type Type = super::NavBar<CategoryT>;
        type ParentType = gtk4::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk4::BoxLayout>();

            klass.set_css_name("navbar");
        }

        type Instance = glib::subclass::basic::InstanceStruct<Self>;
        type Class = glib::subclass::basic::ClassStruct<Self>;
        type Interfaces = ();

        #[inline]
        fn new() -> Self {
            Default::default()
        }
    }

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> ObjectImpl for NavBar<CategoryT> {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            self.navigation_box.set_parent(&*obj);
            self.separators[0].set_parent(&*obj);
            self.stack.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.stack.unparent();
            self.navigation_box.unparent();
            for separator in &self.separators {
                separator.unparent();
            }
        }
    }

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> WidgetImpl for NavBar<CategoryT> {}

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> NavBar<CategoryT> {
        pub fn new() -> Self {
            let separators = [Separator::builder().css_classes(["vertical"]).build()];

            separators[0].add_css_class("highlighted");
            separators[0].set_width_request(3);

            let buttons = RefCell::new(BTreeMap::new());
            let stack = Stack::builder()
                .transition_type(gtk4::StackTransitionType::Crossfade)
                .css_name("main_window")
                .build();

            let navigation_box = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .spacing(0)
                .vexpand(true)
                .vexpand_set(true)
                .baseline_position(gtk4::BaselinePosition::Center)
                .css_name("navbar_buttons_box")
                .build();

            let selected_buttons = Rc::default();

            Self {
                stack,
                navigation_box,
                separators,
                buttons,
                selected_buttons,
            }
        }

        ///
        /// Add the given child to the navigation bars stack.
        /// Also a button labelled with name is added in the corresponding category.
        /// When the button is clicked, the child is shown.
        /// Name has to be unique for a category.
        /// If a callback is present, it is executed when ever the child is shown. This includes when the corresponding button is pressed or `show_child` shows this child.
        ///
        pub fn add_child<F: Fn(&super::NavBar<CategoryT>, &Button, &Stack) + 'static>(
            &self,
            child: &impl IsA<Widget>,
            name: &String,
            category: CategoryT,
            callback: Option<F>,
        ) {
            {
                let buttons_map = self.buttons.borrow();

                // init category if not present
                if !buttons_map.contains_key(&category) {
                    drop(buttons_map);
                    self.init_category(category);
                }
            }

            // add new child to the stack
            self.stack.add_named(child, Some(name.as_str()));

            // and create button for it
            let label = Label::new(Some(name.as_str()));
            let button = Button::builder().child(&label).label(name.clone()).css_name("nav_button").build();

            self.add_button_to_category(&button, category);

            let mut buttons_map = self.buttons.borrow_mut();

            let category_entry = match buttons_map.get_mut(&category) {
                Some(val) => val,
                None => return,
            };

            // show child if according button is clicked
            {
                let stack = &self.stack;
                let sel_buttons = Rc::downgrade(&self.selected_buttons);
                button.connect_clicked(glib::clone!(@weak stack => move |button| {
                    let label = button.label().unwrap();
                    println!("Pressed {}!", label);

                    if let Some(selected_buttons) = sel_buttons.upgrade() {
                        Self::handle_selections(selected_buttons, category, button);
                    }

                    match stack.child_by_name(label.as_str()) {
                        Some(child) => stack.set_visible_child(&child),
                        None => panic!("Cannot find child for name {}, which should be shown", label),
                    }
                }));
            }

            // call the callback whenever the child it is visible
            if let Some(callback) = callback {
                let child_widget: &Widget = child.upcast_ref();
                let obj = self.obj();
                self.stack
                    .connect_visible_child_name_notify(clone!(@weak child_widget, @weak button, @weak obj => move |stack| {
                        if let Some(visible_child) = stack.visible_child() {
                            if  visible_child == child_widget {
                                callback(&obj, &button, stack);
                            }
                        }
                    }));
            }

            // default select the button for the child which is added first
            let mut sel_buttons = self.selected_buttons.borrow_mut();
            if sel_buttons.len() == 0 {
                button.add_css_class("selected");
                sel_buttons.insert(category, button.clone());
            }

            // store new management data
            let res = category_entry.entries.insert(
                name.to_string(),
                NavButton {
                    button,
                    has_stack_child: true,
                },
            );

            debug_assert!(res.is_none());
        }

        ///
        /// Removes the child labelled by name in the category.
        /// Removes the whole category if it is empty afterwards.
        ///
        pub fn remove_child_by_name(&self, name: &String, category: CategoryT) {
            let mut buttons_map = self.buttons.borrow_mut();

            // remove child from stack
            let child = match self.stack.child_by_name(name.as_str()) {
                Some(child) => child,
                None => return,
            };
            self.stack.remove(&child);

            // remove management data and button
            let category_entry_opt = buttons_map.get_mut(&category);
            debug_assert!(category_entry_opt.is_some());
            let category_entry = category_entry_opt.unwrap();
            let button_opt = category_entry.entries.remove(name);
            debug_assert!(button_opt.is_some());
            let button = &button_opt.as_ref().unwrap().button;
            category_entry.button_box.remove(button);

            // remove button from selected button
            let mut sel_buttons = self.selected_buttons.borrow_mut();
            if let Some(sel_button) = sel_buttons.get(&category) {
                if sel_button == button {
                    sel_buttons.remove(&category);
                }
            }

            // remove whole category if not needed
            if category_entry.entries.len() == 0 {
                self.remove_category(category);
            }
        }

        ///
        /// Initializes a category, i.e. creating a separator to the previous buttons
        /// and a box to store the buttons for this category.
        ///
        fn init_category(&self, category: CategoryT) {
            let mut buttons_map = self.buttons.borrow_mut();

            // create box where buttons of this category are stored and map it
            let button_box = GtkBox::builder()
                .orientation(gtk4::Orientation::Vertical)
                .spacing(10)
                .margin_top(0)
                .margin_bottom(0)
                .build();

            let prev_category = Self::prev_category_shown(&buttons_map, category);
            // insert the button_box after the one from the previous category
            self.navigation_box
                .insert_child_after(&button_box, prev_category.map(|category| &buttons_map[&category].button_box));

            let res = buttons_map.insert(
                category,
                CategoryEntry {
                    button_box,
                    separator_before: None,
                    separator_after: None,
                    entries: HashMap::new(),
                    is_shown: true,
                },
            );

            drop(buttons_map);
            self.insert_separator_for_category(category);

            debug_assert!(res.is_none());
        }

        ///
        /// Removes a category, i.e. unmapping separator and button box.
        /// Also removes all remaining children from the stack.
        ///
        fn remove_category(&self, category: CategoryT) {
            let mut buttons_map = self.buttons.borrow_mut();
            let category_entry = match buttons_map.remove(&category) {
                Some(category_entry) => category_entry,
                None => return,
            };

            // remove selected button from this category
            self.selected_buttons.borrow_mut().remove(&category);

            self.navigation_box.remove(&category_entry.button_box);

            // remove now redundant separator
            if let Some(sep_before) = category_entry.separator_before.as_ref() {
                self.navigation_box.remove(sep_before);
            }

            // remove remaining children from stack
            for (name, _) in &category_entry.entries {
                self.remove_child_by_name(name, category);
            }

            if category_entry.is_shown {
                drop(buttons_map);
                self.remove_separator_for_category(category);
            }
        }

        ///
        /// Helper to add a button to a category.
        ///
        fn add_button_to_category(&self, button: &Button, category: CategoryT) {
            let mut buttons_map = self.buttons.borrow_mut();

            let category_entry = match buttons_map.get_mut(&category) {
                Some(val) => val,
                None => return,
            };

            category_entry.button_box.append(button);
        }

        ///
        /// Show child named `name` in `category`. Also show associated button as selected.
        ///
        pub fn show_child(&self, name: &str, category: CategoryT) {
            let buttons = self.buttons.borrow();
            assert!(buttons.contains_key(&category), "The NavBar needs to have a buttons in this category.");
            assert!(
                buttons[&category].entries.contains_key(name),
                "The NavBar needs to have a button with this name!"
            );
            let button = &buttons[&category].entries[name];
            if button.has_stack_child {
                self.stack.set_visible_child_name(name);
            }
            Self::handle_selections(Rc::clone(&self.selected_buttons), category, &button.button);
        }

        ///
        /// Add a custom button without a child on the stack.
        /// The callback can modify the navbar, the button and the stack.
        ///
        pub fn add_custom_nav_button<F: Fn(&super::NavBar<CategoryT>, &Button, &Stack) + 'static>(
            &self,
            name: &str,
            category: CategoryT,
            callback: F,
        ) {
            if !self.buttons.borrow().contains_key(&category) {
                self.init_category(category);
            }

            let label = Label::new(Some(name));
            let custom_button = Button::builder().child(&label).css_name("nav_button").build();
            {
                let stack = &self.stack;
                let obj = &*self.obj();
                let sel_buttons = Rc::downgrade(&self.selected_buttons);
                custom_button.connect_clicked(clone!(@weak obj, @weak stack => move |button| {
                    if let Some(selected_buttons) = sel_buttons.upgrade() {
                        Self::handle_selections(selected_buttons, category, button);
                    }

                    callback(&obj, button, &stack);
                }));
            }
            self.add_button_to_category(&custom_button, category);

            let mut buttons = self.buttons.borrow_mut();
            let category_entry = buttons.get_mut(&category).unwrap();
            category_entry.entries.insert(
                name.into(),
                NavButton {
                    button: custom_button,
                    has_stack_child: false,
                },
            );
        }

        ///
        /// Handles the selections in the categories, i.e. when a new button is clicked/selected.
        /// Marks `clicked_button` as selected and the previously selected button in `clicked_category` as not selected.
        /// Marks the buttons currently selected in the other categories as unselected according to `CategoryT::remaining_selections(clicked_category)`.
        ///
        fn handle_selections(selected_buttons: Rc<RefCell<HashMap<CategoryT, Button>>>, clicked_category: CategoryT, clicked_button: &Button) {
            let mut sel_mut = selected_buttons.borrow_mut();

            let remaining_categories = CategoryT::remaining_selections(clicked_category);

            for category in sel_mut
                .keys()
                .filter(|category| !remaining_categories.contains(category))
                .map(|&category| category)
                .collect::<Vec<CategoryT>>()
            {
                sel_mut.remove(&category).as_ref().unwrap().remove_css_class("selected")
            }

            if let Some(sel_button) = sel_mut.get(&clicked_category) {
                sel_button.remove_css_class("selected");
            }

            // show button as selected
            clicked_button.add_css_class("selected");
            sel_mut.insert(clicked_category, clicked_button.clone());
        }

        ///
        /// Shows the buttons of `category`.
        ///
        pub fn show_category(&self, category: CategoryT) {
            let mut buttons = self.buttons.borrow_mut();
            if let Some(category_entry) = buttons.get(&category) {
                if !category_entry.is_shown {
                    let prev_category = Self::prev_category_shown(&*buttons, category);
                    self.navigation_box
                        .insert_child_after(&category_entry.button_box, prev_category.map(|prev_cat| &buttons[&prev_cat].button_box));
                    buttons.get_mut(&category).unwrap().is_shown = true;
                    drop(buttons);
                    self.insert_separator_for_category(category);
                }
            }
        }

        ///
        /// Hides the buttons of `category`.
        ///
        pub fn hide_category(&self, category: CategoryT) {
            let mut buttons = self.buttons.borrow_mut();
            if let Some(category_entry) = buttons.get_mut(&category) {
                if category_entry.is_shown {
                    category_entry.button_box.unparent();
                    category_entry.is_shown = false;
                    drop(buttons);
                    self.remove_separator_for_category(category);
                }
            }
        }

        ///
        /// Inserts a separator before the category if necessary.
        /// If it is the first category, the separator will be inserted after it if necessary.
        /// If there is no other category shown, no separator will be inserted.
        ///
        fn insert_separator_for_category(&self, category: CategoryT) {
            let mut buttons = self.buttons.borrow_mut();
            let (separator_before, separator_after) = match (
                Self::prev_category_shown(&*buttons, category),
                Self::next_category_shown(&*buttons, category),
            ) {
                (None, None) => (None, None), // there is no other shown category, no separator necessary
                (Some(prev_cat), None) => {
                    // this category will be the new last category => separator to previous category
                    let new_sep = Self::create_nav_bar_separator();
                    buttons.get_mut(&prev_cat).unwrap().separator_after = Some(new_sep.clone());
                    (Some(new_sep), None)
                }
                (None, Some(next_cat)) => {
                    // this category wll be the new first category => separator to next category
                    let new_sep = Self::create_nav_bar_separator();
                    buttons.get_mut(&next_cat).unwrap().separator_before = Some(new_sep.clone());
                    (None, Some(new_sep))
                }
                (Some(prev_cat), Some(_)) => {
                    // this category will be in the middle => new separator to other categories, adjust separator pointer
                    let new_sep = Self::create_nav_bar_separator();
                    let sep_after = mem::replace(&mut buttons.get_mut(&prev_cat).unwrap().separator_after, Some(new_sep.clone()));
                    (Some(new_sep), sep_after)
                }
            };

            let category_entry = buttons.get_mut(&category).unwrap();

            // insert a new separator before the button_box
            if let Some(new_sep) = separator_before.as_ref() {
                new_sep.insert_before(&self.navigation_box, Some(&category_entry.button_box));
            }

            category_entry.separator_before = separator_before;
            category_entry.separator_after = separator_after;
        }

        ///
        /// Removes the separator before a category if available.
        /// If the category is the first one, the separator after it will be removed.
        /// If no separator is available, none will be removed.
        ///
        fn remove_separator_for_category(&self, category: CategoryT) {
            let mut buttons = self.buttons.borrow_mut();
            let removed_sep = match (
                Self::prev_category_shown(&buttons, category),
                Self::next_category_shown(&buttons, category),
            ) {
                (None, None) => None, // this is/was only shown category, nothing to do
                (Some(prev_cat), None) => mem::take(&mut buttons.get_mut(&prev_cat).unwrap().separator_after), // this was the last shown category
                (None, Some(next_cat)) => mem::take(&mut buttons.get_mut(&next_cat).unwrap().separator_before), // this was the first shown category
                (Some(prev_cat), Some(next_cat)) => {
                    // somewhere in the middle

                    let next_before_separator = buttons.get_mut(&next_cat).unwrap().separator_before.clone();
                    let removed_sep = mem::replace(&mut buttons.get_mut(&prev_cat).unwrap().separator_after, next_before_separator);

                    removed_sep
                }
            };

            if let Some(removed_sep) = removed_sep {
                removed_sep.unparent();
            }

            let category_entry = buttons.get_mut(&category).unwrap();
            category_entry.separator_before = None;
            category_entry.separator_after = None;
        }

        ///
        /// Returns the previous category in the ordering of the categories which is shown.
        /// If there is none, `None` is returned.
        ///
        fn prev_category_shown(buttons: &BTreeMap<CategoryT, CategoryEntry>, category: CategoryT) -> Option<CategoryT> {
            buttons
                .iter()
                .filter(|(&cat, entry)| cat < category && entry.is_shown)
                .map(|(&cat, _)| cat)
                .last()
        }

        ///
        /// Returns the next category in the ordering of the categories which is shown.
        /// If there is none, `None` is returned.
        ///
        fn next_category_shown(buttons: &BTreeMap<CategoryT, CategoryEntry>, category: CategoryT) -> Option<CategoryT> {
            buttons
                .iter()
                .filter(|(&cat, entry)| cat > category && entry.is_shown)
                .map(|(&cat, _)| cat)
                .next()
        }

        ///
        /// Creates a separator used in the `navigation_box`.
        ///
        fn create_nav_bar_separator() -> Separator {
            let sep = Separator::builder().margin_top(5).margin_bottom(5).build();
            sep.add_css_class("category_sep");
            sep
        }

        ///
        /// Returns whether a category is currently shown.
        ///
        pub fn is_category_shown(&self, category: CategoryT) -> bool {
            self.buttons.borrow().get(&category).map_or(false, |entry| entry.is_shown)
        }

        ///
        /// Returns which categories are currently selected.
        ///
        pub fn get_selected_categories(&self) -> HashSet<CategoryT> {
            let selected_buttons = self.selected_buttons.borrow();
            selected_buttons.iter().map(|(category, _)| *category).collect()
        }
    }

    ///
    /// Stores management information about a category.
    ///
    #[derive(Debug)]
    struct CategoryEntry {
        /// The box which stores the nav buttons of this category.
        button_box: GtkBox,
        /// The separator before this category to separate it from the previous category. May not be present, i.e. first category.
        separator_before: Option<Separator>,
        /// The separator after this category to separate if from the next category. May not be present, i.e. last category.
        separator_after: Option<Separator>,
        /// The navigation buttons associated to this category, maps from button name to the button.
        entries: HashMap<String, NavButton>,
        /// Describes if the category is currently shown in the navigation box
        is_shown: bool,
    }

    ///
    /// Stores management information about a button used as nav_button.
    ///
    #[derive(Debug)]
    struct NavButton {
        /// The actual button.
        button: Button,
        /// Describes whether a child with the same name is stored on the stack and should be popped from the stack, when removing the button.
        has_stack_child: bool,
    }
}

// TODO: Maybe use Panes: https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.Paned.html
glib::wrapper! {
    pub struct NavBar<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait>(ObjectSubclass<inner::NavBar<CategoryT>>)
        @extends gtk4::Widget;
}

impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> NavBar<CategoryT> {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }

    ///
    /// Add the given child to the navigation bars stack.
    /// Also a button labelled with name is added in the corresponding category.
    /// When the button is clicked, the child is shown.
    /// Name has to be unique for a category.
    ///
    pub fn add_child(&self, child: &impl IsA<Widget>, name: String, category: CategoryT) {
        self.imp().add_child(child, &name, category, None::<fn(&Self, &Button, &Stack)>);
    }

    ///
    /// Same as `add_child`, except `callback` is called whenever the child is shown.
    ///
    pub fn add_child_with_callback<F: Fn(&Self, &Button, &Stack) + 'static>(
        &self,
        child: &impl IsA<Widget>,
        name: String,
        category: CategoryT,
        callback: F,
    ) {
        self.imp().add_child(child, &name, category, Some(callback));
    }

    ///
    /// Add a custom button without a child on the stack.
    /// The callback can modify the navbar, the button and the stack.
    ///
    pub fn add_custom_nav_button<F: Fn(&Self, &Button, &Stack) + 'static>(&self, name: &str, category: CategoryT, callback: F) {
        self.imp().add_custom_nav_button(name, category, callback);
    }

    ///
    /// Removes the child labelled by name in the category.
    /// Removes the whole category if it is empty afterwards.
    ///
    pub fn remove_child_by_name(&self, name: String, category: CategoryT) {
        self.imp().remove_child_by_name(&name, category);
    }

    ///
    /// Show child named `name` in `category`. Also show associated button as selected.
    ///
    pub fn show_child(&self, name: &str, category: CategoryT) {
        self.imp().show_child(name, category);
    }

    ///
    /// Shows the buttons of `category`.
    ///
    pub fn show_category(&self, category: CategoryT) {
        self.imp().show_category(category);
    }

    ///
    /// Hides the buttons of `category`.
    ///
    pub fn hide_category(&self, category: CategoryT) {
        self.imp().hide_category(category);
    }

    ///
    /// Returns whether `category` is currently shown.
    ///
    pub fn is_category_shown(&self, category: CategoryT) -> bool {
        self.imp().is_category_shown(category)
    }

    ///
    /// Returns which categories are currently selected.
    ///
    pub fn get_selected_categories(&self) -> HashSet<CategoryT> {
        self.imp().get_selected_categories()
    }
}

///
/// Defines the interface a category used in `NavBar` has to fulfill.
///
pub trait NavBarCategoryTrait: Sized {
    ///
    /// Returns the categories, in which the selection should remain.
    /// `selected_category` should also contained in the return value.
    ///
    fn remaining_selections(selected_category: Self) -> Vec<Self>;

    /// The name of the CategoryType.
    const NAME: &'static str;
    /// The name used to describe the NavBar when using with this CategoryType.
    const NAV_BAR_NAME: &'static str;
}
