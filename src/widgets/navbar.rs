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
use gtk4::{
    glib, subclass::widget::*, traits::*, Box as GtkBox, Button, Label, Separator, Stack, Widget,
};
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Once;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
};

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct NavBar<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> {
        /// Stores the children and only showing one at a time.
        stack: Stack,
        /// Different separators between the main widgets.
        /// [0] = separator between `navigation_box` and `stack`
        separators: [Separator; 1],
        // Contains the buttons of the categories as well as the separators between the categories (not to be confused with `separators`)
        navigation_box: GtkBox,
        // Stores the information about each category, i.e. what buttons and if a separator to the previous category exists
        buttons: RefCell<BTreeMap<CategoryT, CategoryEntry>>,
        // Stores which button in each category is selected, it is not necessary for a category to have a selected button
        selected_buttons: Rc<RefCell<HashMap<CategoryT, Button>>>,
    }

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> Default for NavBar<CategoryT> {
        fn default() -> Self {
            Self::new()
        }
    }

    unsafe impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> ObjectSubclassType
        for NavBar<CategoryT>
    {
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
    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> FromObject
        for NavBar<CategoryT>
    {
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
    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> ::std::borrow::ToOwned
        for NavBar<CategoryT>
    {
        type Owned = ObjectImplRef<NavBar<CategoryT>>;

        #[inline]
        fn to_owned(&self) -> Self::Owned {
            ObjectSubclassExt::ref_counted(self)
        }
    }

    #[doc(hidden)]
    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait>
        ::std::borrow::Borrow<NavBar<CategoryT>> for ObjectImplRef<NavBar<CategoryT>>
    {
        #[inline]
        fn borrow(&self) -> &NavBar<CategoryT> {
            self
        }
    }

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> ObjectSubclass
        for NavBar<CategoryT>
    {
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

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> ObjectImpl
        for NavBar<CategoryT>
    {
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

    impl<CategoryT: 'static + Hash + Ord + Copy + NavBarCategoryTrait> WidgetImpl
        for NavBar<CategoryT>
    {
    }

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
        ///
        pub fn add_child(&self, child: &impl IsA<Widget>, name: &String, category: CategoryT) {
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
            let button = Button::builder()
                .child(&label)
                .label(name.clone())
                .css_name("nav_button")
                .build();

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
            self.navigation_box.append(&button_box);

            let res = buttons_map.insert(
                category,
                CategoryEntry {
                    button_box,
                    separator: None,
                    entries: HashMap::new(),
                },
            );

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

            if category_entry.separator.is_some() {
                self.navigation_box
                    .remove(category_entry.separator.as_ref().unwrap());
            }

            // remove remaining children from stack
            for (name, _) in &category_entry.entries {
                self.remove_child_by_name(name, category);
            }
        }

        ///
        /// Helper to add a button to a category.
        ///
        fn add_button_to_category(&self, button: &Button, category: CategoryT) {
            let mut buttons_map = self.buttons.borrow_mut();

            let count_categories_before: u32 = buttons_map
                .keys()
                .filter_map(|&key| if key < category { Some(1) } else { None })
                .sum();

            let category_entry = match buttons_map.get_mut(&category) {
                Some(val) => val,
                None => return,
            };

            // create separator to separate from previous category if necessary
            if category_entry.separator.is_none() && count_categories_before > 0 {
                let sep = Separator::builder().margin_top(5).margin_bottom(5).build();
                sep.add_css_class("category_sep");
                sep.insert_before(&self.navigation_box, Some(&category_entry.button_box));
                category_entry.separator = Some(sep);
            }

            category_entry.button_box.append(button);
        }

        ///
        /// Show child named `name` in `category`. Also show associated button as selected.
        ///
        pub fn show_child(&self, name: &str, category: CategoryT) {
            let mut sel_buttons = self.selected_buttons.borrow_mut();
            let buttons = self.buttons.borrow();

            // show previously selected button as not selected
            if let Some(sel_button) = sel_buttons.get(&category) {
                sel_button.remove_css_class("selected");
            }

            // show associated button as selected and show child
            if let Some(button) = buttons[&category].entries.get(name) {
                button.button.add_css_class("selected");
                sel_buttons.insert(category, button.button.clone());

                debug_assert!(button.has_stack_child);
                if button.has_stack_child {
                    self.stack.set_visible_child_name(name);
                }
            }
        }

        ///
        /// Add a custom button without a child on the stack.
        /// The callback can modify the navbar, the button and the stack.
        ///
        pub fn add_custom_nav_button<
            F: Fn(&super::NavBar<CategoryT>, &Button, &Stack) + 'static,
        >(
            &self,
            name: &str,
            category: CategoryT,
            callback: F,
        ) {
            if !self.buttons.borrow().contains_key(&category) {
                self.init_category(category);
            }

            let label = Label::new(Some(name));
            let custom_button = Button::builder()
                .child(&label)
                .css_name("nav_button")
                .build();
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
        fn handle_selections(
            selected_buttons: Rc<RefCell<HashMap<CategoryT, Button>>>,
            clicked_category: CategoryT,
            clicked_button: &Button,
        ) {
            let mut sel_mut = selected_buttons.borrow_mut();

            let remaining_categories = CategoryT::remaining_selections(clicked_category);
            sel_mut
                .iter()
                .filter(|(category, _)| !remaining_categories.contains(category))
                .for_each(|(_, sel_button)| sel_button.remove_css_class("selected"));

            if let Some(sel_button) = sel_mut.get(&clicked_category) {
                sel_button.remove_css_class("selected");
            }

            // show button as selected
            clicked_button.add_css_class("selected");
            sel_mut.insert(clicked_category, clicked_button.clone());
        }
    }

    ///
    /// Stores management information about a category.
    ///
    #[derive(Debug)]
    struct CategoryEntry {
        /// The box which stores the nav buttons of this category.
        button_box: GtkBox,
        /// The separator before this category to separate it from the previous one. May not be present, i.e. first category.
        separator: Option<Separator>,
        /// The navigation buttons associated to this category, maps from button name to the button.
        entries: HashMap<String, NavButton>,
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
        self.imp().add_child(child, &String::from(name), category);
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
    /// Add a custom button without a child on the stack.
    /// The callback can modify the navbar, the button and the stack.
    ///
    pub fn add_custom_nav_button<F: Fn(&Self, &Button, &Stack) + 'static>(
        &self,
        name: &str,
        category: CategoryT,
        callback: F,
    ) {
        self.imp().add_custom_nav_button(name, category, callback);
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

    const NAME: &'static str;
    const NAV_BAR_NAME: &'static str;
}
