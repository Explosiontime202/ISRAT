use self::fix_indexed_list_store::FixIndexedListStore;
use fix_indexed_list_entry::FixIndexedListEntry;
use gdk4::glib::{
    clone,
    clone::Downgrade,
    closure_local,
    once_cell::sync::Lazy,
    subclass::{types::FromObject, *},
    Object, Type, Value,
};
use gdk4::{ContentFormats, ContentProvider, DragAction};
use gtk4::{
    glib, prelude::*, subclass::prelude::*, Box as GtkBox, Button, CenterBox, DragSource, DropTarget, Image, Label, LayoutManager, ListBox,
    ListBoxRow, Orientation, ScrolledWindow, SelectionMode, Widget, WidgetPaintable,
};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    mem,
    sync::Once,
};

pub mod fix_indexed_list_entry;
pub mod fix_indexed_list_store;

/// The direction in which the fix indexed list entries should be rotated.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RotateDirection {
    Up,
    Down,
}

mod inner {
    use super::*;

    #[derive(Debug)]
    pub struct FixIndexedList<
        DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static,
        const TYPE_NAME: &'static str,
        const ENTRY_TYPE_NAME: &'static str,
    > {
        /// The actual box displaying the rows.
        list_box: ListBox,
        /// Stores the rows displayed in the list.
        pub model: FixIndexedListStore<DataType, ENTRY_TYPE_NAME>,
        /// Contains the ListBox. (a direct child)
        scrolled_window: ScrolledWindow,
        /// Contains the button to append a row. Only present if `allow_count_changes` is set. (a direct child)
        append_row_button: RefCell<Option<CenterBox>>,
        /// Management information: The row above the currently hovered/highlighted gap during drag'n'drop
        /// None if no drag'n'drop is active or the gap above the first list element is hovered/highlighted.
        row_above: RefCell<Option<ListBoxRow>>,
        /// Management information: The row below the currently hovered/highlighted gap during drag'n'drop.
        /// None if no drag'n'drop is active or the gap below the last list element is hovered/highlighted.
        row_below: RefCell<Option<ListBoxRow>>,
        /// Describes whether rows can be appended and removed or not. The according widgets will be visible or not.
        allow_count_changes: Cell<bool>,
    }

    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        Default for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
        fn default() -> Self {
            let scrolled_window = ScrolledWindow::builder()
                .propagate_natural_height(true)
                .hscrollbar_policy(gtk4::PolicyType::Never)
                .build();

            Self {
                list_box: ListBox::builder().selection_mode(SelectionMode::None).build(),
                model: Default::default(),
                scrolled_window,
                append_row_button: Default::default(),
                row_above: Default::default(),
                row_below: Default::default(),
                allow_count_changes: Cell::new(false),
            }
        }
    }

    // ----- begin of macro expansion of glib::object_subclass -----
    unsafe impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        ObjectSubclassType for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
        #[inline]
        fn type_data() -> ::std::ptr::NonNull<TypeData> {
            // Make sure to keep type data for every generic type. CAUTION: this differs glib::object_subclass proc macro.
            static mut DATA_MAP: Lazy<Vec<(Type, TypeData)>> = Lazy::new(|| Vec::new());
            unsafe {
                if !DATA_MAP.iter().any(|(key, _)| key == &DataType::static_type()) {
                    DATA_MAP.push((DataType::static_type(), types::INIT_TYPE_DATA));
                }
                ::std::ptr::NonNull::from(&mut DATA_MAP.iter_mut().find(|(key, _)| key == &DataType::static_type()).unwrap().1)
            }
        }

        #[inline]
        fn type_() -> Type {
            // Make sure to register the type for every generic. CAUTION: this differs glib::object_subclass proc macro.
            static mut ONCE_MAP: Lazy<HashMap<Type, Once>> = Lazy::new(|| HashMap::new());

            unsafe {
                if !ONCE_MAP.contains_key(&DataType::static_type()) {
                    ONCE_MAP.insert(DataType::static_type(), Once::new());
                }
                ONCE_MAP[&DataType::static_type()].call_once(|| {
                    register_type::<Self>();
                })
            }
            unsafe {
                let data = Self::type_data();
                let type_ = data.as_ref().type_();

                assert_eq!(Self::NAME, type_.to_string());

                type_
            }
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        FromObject for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
        type FromObjectType = <Self as ObjectSubclass>::Type;
        #[inline]
        fn from_object(obj: &Self::FromObjectType) -> &Self {
            <Self as ObjectSubclassExt>::from_obj(obj)
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        Downgrade for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
        type Weak = ObjectImplWeakRef<FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>>;

        #[inline]
        fn downgrade(&self) -> Self::Weak {
            let ref_counted = ObjectSubclassExt::ref_counted(self);
            Downgrade::downgrade(&ref_counted)
        }
    }

    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
        #[inline]
        pub fn downgrade(&self) -> <Self as Downgrade>::Weak {
            Downgrade::downgrade(self)
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        ::std::borrow::ToOwned for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
        type Owned = ObjectImplRef<FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>>;

        #[inline]
        fn to_owned(&self) -> Self::Owned {
            ObjectSubclassExt::ref_counted(self)
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        ::std::borrow::Borrow<FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>>
        for ObjectImplRef<FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>>
    {
        #[inline]
        fn borrow(&self) -> &FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> {
            self
        }
    }

    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        ObjectSubclass for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
        const NAME: &'static str = TYPE_NAME;
        type Type = super::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>;
        type ParentType = gtk4::Widget;

        type Instance = glib::subclass::basic::InstanceStruct<Self>;
        type Class = glib::subclass::basic::ClassStruct<Self>;
        type Interfaces = ();

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk4::BoxLayout>();
            klass.set_css_name("fixed_indexed_list");
        }

        #[inline]
        fn new() -> Self {
            Default::default()
        }
    }
    // ----- end of macro expansion of glib::object_subclass -----

    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        ObjectImpl for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.property::<LayoutManager>("layout_manager")
                .set_property("orientation", Orientation::Vertical);

            self.list_box.bind_model(
                Some(&self.model.list_store),
                clone!(@weak self as this => @default-panic, move |object| {
                   let data: &FixIndexedListEntry<DataType, ENTRY_TYPE_NAME> = object.downcast_ref().expect("Should have type DataType!");
                   this.create_row(data)
                }),
            );

            self.setup_drop_target();

            // only show the ListBox if there is something to show
            self.obj().connect_row_count(clone!(@weak self as this => move |_, row_count| {
                match row_count {
                    0 => this.scrolled_window.set_visible(false),
                    1 => this.scrolled_window.set_visible(true),
                    _ => ()
                };
            }));

            self.scrolled_window.set_child(Some(&self.list_box));
            self.scrolled_window.set_visible(false);
            self.scrolled_window.set_parent(&*obj);
        }

        fn dispose(&self) {
            self.scrolled_window.unparent();
            if let Some(center_widget) = self.append_row_button.borrow().as_ref() {
                center_widget.unparent();
            }
        }

        fn signals() -> &'static [Signal] {
            // Caution: This differs massively from the "normal" way of defining `signals()`
            // Setup signals for each DataType separately
            static mut SIGNALS_MAP: Lazy<HashMap<Type, Vec<Signal>>> = Lazy::new(|| HashMap::new());

            unsafe {
                if !SIGNALS_MAP.contains_key(&DataType::static_type()) {
                    SIGNALS_MAP.insert(
                        DataType::static_type(),
                        vec![
                            // emitted when the FixIndexedList requests the using widget to create a new widget for a given data from type DataType
                            Signal::builder("create-data-widget")
                                .param_types([u32::static_type(), DataType::static_type()])
                                .return_type::<Widget>()
                                .build(),
                            // emitted when the FixIndexedList requests the using widget to create a new data object of type DataType
                            Signal::builder("create-data-object").return_type::<DataType>().build(),
                            // emitted when the FixIndexedList requests the using widget to create a new widget which will be used as label in the "append" button
                            Signal::builder("create-append-widget").return_type::<Widget>().build(),
                            // emitted when the row count changed, i.e. a row was appended or removed
                            Signal::builder("row-count").param_types([u32::static_type()]).build(),
                            // emitted when a row is appended
                            Signal::builder("row-appended").build(),
                            // emitted when a row is removed; also provides the removed data object of type DataType
                            Signal::builder("row-removed").param_types([DataType::static_type()]).build(),
                        ],
                    );
                }
                &SIGNALS_MAP[&DataType::static_type()]
            }
        }
    }

    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        WidgetImpl for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
    }

    impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
        FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
    {
        ///
        /// Create the widgets for a row containing the `data`.
        ///
        pub fn create_row(&self, data: &FixIndexedListEntry<DataType, ENTRY_TYPE_NAME>) -> Widget {
            let row = GtkBox::new(Orientation::Horizontal, 10);

            let dnd_icon = Image::from_icon_name("open-menu-symbolic");
            let drag_source = DragSource::builder().actions(DragAction::MOVE).build();

            let number_label = Label::builder()
                .label((data.get_position() + 1).to_string())
                .justify(gtk4::Justification::Center)
                .build();

            // request a new data widget from the using widget
            let data_widget = self.obj().emit_create_data_widget(data.get_position(), data.get_data());

            row.append(&dnd_icon);
            row.append(&number_label);
            row.append(&data_widget);

            let list_box_row = ListBoxRow::builder().child(&row).focusable(false).build();

            if self.allow_count_changes.get() {
                let remove_button = Button::builder().icon_name("list-remove").focusable(false).build();
                remove_button.connect_clicked(clone!(@weak self as this, @weak list_box_row => move |_| {
                    this.remove_row(list_box_row.index() as u32);
                }));
                row.append(&remove_button);
            }

            drag_source.connect_prepare(clone!(@weak data, @weak row => @default-panic, move |drag_source, x, y| {
                drag_source.set_icon(Some(&WidgetPaintable::new(Some(&row))), x as i32, y as i32);

                let content_provider = ContentProvider::for_value(&data.to_value());
                Some(content_provider)
            }));

            dnd_icon.add_controller(drag_source);
            list_box_row.into()
        }

        ///
        /// Creates the DropTarget and adds it as controller to the ListBox.
        /// Calls `handle_drop` when data is dropped onto the drop target.
        ///
        fn setup_drop_target(&self) {
            let drop_target = DropTarget::builder()
                .formats(&ContentFormats::for_type(FixIndexedListEntry::<DataType, ENTRY_TYPE_NAME>::static_type()))
                .actions(DragAction::MOVE)
                .build();

            drop_target.connect_accept(|_, drop| {
                // only accept if drag is from this application
                drop.drag().is_some()
            });

            drop_target.connect_drop(clone!(@weak self as this => @default-return false, move |_, val, _, _| {
                if !this.check_valid_drop(val) {
                    this.unmark_above_below_row();
                    return false;
                }

                let dropped_data : FixIndexedListEntry<DataType, ENTRY_TYPE_NAME> = val.get().unwrap();

                this.handle_drop(&dropped_data);
                this.unmark_above_below_row();
                true
            }));

            drop_target.connect_enter(clone!(@weak self as this => @default-return DragAction::empty(), move |_, _, y| {
                this.update_above_below_row(y as i32);
                DragAction::MOVE
            }));

            drop_target.connect_motion(clone!(@weak self as this => @default-return DragAction::empty(), move |_, _, y| {
                this.unmark_above_below_row();
                this.update_above_below_row(y as i32);
                DragAction::MOVE
            }));

            drop_target.connect_leave(clone!(@weak self as this => move |_| {
                this.unmark_above_below_row();
            }));

            self.list_box.add_controller(drop_target);
        }

        ///
        /// Handle the dropped data, i.e. moving the entry at the source position to the destination position (i.e. the point where it was dropped).
        /// All other entries between are shifted upwards or downwards, depending in which direction the row was moved.
        ///
        fn handle_drop(&self, data: &FixIndexedListEntry<DataType, ENTRY_TYPE_NAME>) {
            let row_above = self.row_above.borrow();
            let row_below = self.row_below.borrow();
            let src_idx = data.get_position() as u32;

            // determine the direction in which the row was moved as well as the row which will contain the data after the drop has been finished.
            let (rot_dir, dst_idx) = match (row_above.as_ref(), row_below.as_ref()) {
                (None, None) => panic!("Should not happen!"),
                (Some(_), None) => (RotateDirection::Down, self.model.n_items() - 1),
                (None, Some(_)) => (RotateDirection::Up, 0),
                (Some(prev_row), Some(next_row)) => {
                    // element was moved somewhere in between, move direction has to be calculated
                    let dst_idx = next_row.index();
                    let diff = dst_idx - src_idx as i32;
                    assert_ne!(diff, 0, "Difference between source {src_idx} and destination {dst_idx} cannot be zero!");
                    if diff < 0 {
                        (RotateDirection::Up, next_row.index() as u32)
                    } else {
                        (RotateDirection::Down, prev_row.index() as u32)
                    }
                }
            };

            // move row into its new position
            self.model.rotate_entries(src_idx, dst_idx, rot_dir);
        }

        ///
        /// Checks that the dropped data is valid and the drop should be handled further.
        /// Drops are rejected if the data has a wrong type or no move has to happen, i.e. the row is dropped right above or below itself.
        ///
        fn check_valid_drop(&self, value: &glib::Value) -> bool {
            // accept the drop only if a correct value type is dropped
            let drop_data: FixIndexedListEntry<DataType, ENTRY_TYPE_NAME> = match value.get() {
                Ok(drop_data) => drop_data,
                Err(_) => return false,
            };

            let src_idx = drop_data.get_position() as u32;
            let row_above = self.row_above.borrow();
            let row_below = self.row_below.borrow();

            // reject drags if the source row is either directly above or directly below the dropped position, i.e. no moving is necessary

            if let Some(prev_row) = row_above.as_ref() {
                let prev_pos = prev_row.index() as u32;
                if src_idx == prev_pos {
                    drop(row_above);
                    drop(row_below);
                    return false;
                }
            }

            if let Some(next_row) = row_below.as_ref() {
                let next_pos = next_row.index() as u32;
                if src_idx == next_pos {
                    drop(row_above);
                    drop(row_below);
                    return false;
                }
            }
            true
        }

        ///
        /// Updates `row_above` and `row_below` according to the `y` coordinate where the mouse during drag and drop is currently hovering.
        ///
        fn update_above_below_row(&self, y: i32) {
            // get the row where the mouse is hovering
            if let Some(row) = self.list_box.row_at_y(y) {
                let allocation = row.allocation();
                let row_before: Option<ListBoxRow>;
                let row_after: Option<ListBoxRow>;

                if y < (allocation.y() + allocation.height() / 2) {
                    // if hovering in the upper half of the row, the gap above this row is highlighted
                    row_before = row.prev_sibling().map(|widget| widget.downcast().unwrap());
                    row_after = Some(row);
                } else {
                    // if hovering in the lower half of the row, the gap below this row is highlighted
                    row_after = row.next_sibling().map(|widget| widget.downcast().unwrap());
                    row_before = Some(row);
                }

                // set the css classes in order to highlight the corresponding top and bottom borders
                if let Some(prev_row) = row_before.as_ref() {
                    prev_row.add_css_class("drag-hover-bottom");
                }
                if let Some(next_row) = row_after.as_ref() {
                    next_row.add_css_class("drag-hover-top");
                }

                *self.row_above.borrow_mut() = row_before;
                *self.row_below.borrow_mut() = row_after;
            }
        }

        ///
        /// Remove the highlighting of the gap.
        ///
        fn unmark_above_below_row(&self) {
            let mut row_before = self.row_above.borrow_mut();
            if let Some(prev_row) = mem::replace(&mut *row_before, None) {
                prev_row.remove_css_class("drag-hover-bottom");
            }

            let mut row_after = self.row_below.borrow_mut();
            if let Some(next_row) = mem::replace(&mut *row_after, None) {
                next_row.remove_css_class("drag-hover-top");
            }
        }

        ///
        /// Adds a new row to the list. Emits the signal "create-data-object" in order to request a new data object to be inserted in the new slot.
        /// Emits the signals "row-count" and "row-appended".
        ///
        fn append_row(&self) {
            assert!(self.allow_count_changes.get());
            let data_obj = self.obj().emit_create_data_object();
            self.append_row_with_object(data_obj);
        }

        ///
        /// Adds a new row to the list. The signal "create-data-object" is not emitted, but the given object is used.
        /// Emits the signals "row-count" and "row-appended".
        ///
        pub fn append_row_with_object(&self, data_object: DataType) {
            self.model.append(data_object);
            self.obj().emit_row_appended();
            self.obj().emit_row_count(self.model.n_items());
        }

        ///
        /// Removes the row at `idx` if possible and emits the signals "row-removed" and "row-count".
        ///
        fn remove_row(&self, idx: u32) {
            assert!(self.allow_count_changes.get());
            let data_opt = self.model.get(idx);
            if let Some(data) = data_opt {
                let res = self.model.remove(idx);
                debug_assert!(res);
                self.obj().emit_row_removed(data);
                self.obj().emit_row_count(self.model.n_items());
            }
        }

        ///
        /// Sets `allow_count_changes` and creates the append row button if set to `true`.
        ///
        pub fn set_allow_count_changes(&self, allow_count_changes: bool) {
            self.allow_count_changes.set(allow_count_changes);
            if allow_count_changes {
                self.setup_append_row_button();
            }
        }

        ///
        /// Creates the append row button. Uses the signal "create-append-widget" to get the label of the append button.
        ///
        fn setup_append_row_button(&self) {
            let append_center_box = CenterBox::builder().css_name("team_add_center").build();
            let append_team_button = Button::builder().child(&append_center_box).focusable(false).build();
            append_center_box.set_start_widget(Some(&Image::from_icon_name("list-add")));

            // TODO: Check if signal was connected or else use default value for label
            let label = self.obj().emit_create_append_widget();
            append_center_box.set_end_widget(Some(&label));

            append_team_button.connect_clicked(clone!(@weak self as this => move |_| {
                this.append_row();
            }));

            let outer_center_box = CenterBox::new();
            outer_center_box.set_center_widget(Some(&append_team_button));
            outer_center_box.set_parent(&*self.obj());

            *self.append_row_button.borrow_mut() = Some(outer_center_box);
        }
    }
}

// The implementation is above the struct definition in order to keep the actual important part visible and not hidden in the mess below.
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    ///
    /// Before any widget is appended, the signal "create-data-widget" must be connected!
    ///
    pub fn new() -> Self {
        Object::new::<Self>()
    }

    ///
    /// Creates a FixIndexedList with the given objects.
    /// `create_data_widgets_func` is connected to the signal "create-data-widget".
    ///
    pub fn with_default_objects<F: Fn(&Self, u32, &DataType) -> Widget + 'static>(data_objects: Vec<DataType>, create_data_widgets_func: F) -> Self {
        let obj = Self::new();
        obj.connect_create_data_widget(create_data_widgets_func);
        for data_object in data_objects {
            obj.imp().append_row_with_object(data_object);
        }
        obj
    }

    ///
    /// `allow_count_changes` specifies whether rows can be appended and removed.
    /// The according widgets are shown or not.
    /// The default value is false.
    ///
    /// If setting `true`, this method has to be called **after** `connect_create_append_widget`
    /// in order to be able to create the widget for the append button.
    ///
    #[inline]
    pub fn set_allow_count_changes(&self, allow_count_changes: bool) {
        self.imp().set_allow_count_changes(allow_count_changes);
    }

    #[inline]
    pub fn connect_create_data_widget<F: Fn(&Self, u32, &DataType) -> Widget + 'static>(&self, f: F) {
        self.connect_closure(
            "create-data-widget",
            true,
            closure_local!(move |list: &Self, position: u32, data: DataType| { f(list, position, &data) }),
        );
    }

    #[inline]
    pub fn connect_create_data_object<F: Fn(&Self) -> DataType + 'static>(&self, f: F) {
        self.connect_closure("create-data-object", true, closure_local!(move |list: &Self| { f(list) }));
    }

    #[inline]
    pub fn connect_create_append_widget<F: Fn(&Self) -> Widget + 'static>(&self, f: F) {
        self.connect_closure("create-append-widget", true, closure_local!(move |list: &Self| { f(list) }));
    }

    #[inline]
    pub fn connect_row_count<F: Fn(&Self, u32) + 'static>(&self, f: F) {
        self.connect_closure(
            "row-count",
            true,
            closure_local!(move |list: &Self, row_count: u32| {
                f(list, row_count);
            }),
        );
    }

    #[inline]
    pub fn connect_row_appended<F: Fn(&Self) + 'static>(&self, f: F) {
        self.connect_closure(
            "row-appended",
            true,
            closure_local!(move |list: &Self| {
                f(list);
            }),
        );
    }

    #[inline]
    pub fn connect_row_removed<F: Fn(&Self, &DataType) + 'static>(&self, f: F) {
        self.connect_closure(
            "row-removed",
            true,
            closure_local!(move |list: &Self, data: DataType| {
                f(list, &data);
            }),
        );
    }

    #[inline]
    pub fn emit_create_data_widget(&self, position: u32, data: DataType) -> Widget {
        self.emit_by_name("create-data-widget", &[&position.to_value(), &data.to_value()])
    }

    #[inline]
    pub fn emit_create_data_object(&self) -> DataType {
        self.emit_by_name("create-data-object", &[])
    }

    #[inline]
    pub fn emit_create_append_widget(&self) -> Widget {
        self.emit_by_name("create-append-widget", &[])
    }

    #[inline]
    pub fn emit_row_count(&self, row_count: u32) {
        let _: () = self.emit_by_name("row-count", &[&row_count.to_value()]);
    }

    #[inline]
    pub fn emit_row_appended(&self) {
        let _: () = self.emit_by_name("row-appended", &[]);
    }

    #[inline]
    pub fn emit_row_removed(&self, data: DataType) {
        let _: () = self.emit_by_name("row-removed", &[&data.to_value()]);
    }

    pub fn get_model(&self) -> &FixIndexedListStore<DataType, ENTRY_TYPE_NAME> {
        &self.imp().model
    }
}

// This mess below is the expansion of the following macro, but as this does not work out of the box, we expanded it manually.
// Caution: Can break with newer versions of this macro.
/*glib::wrapper!{
    pub struct FixIndexedList<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str >(ObjectSubclass<inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>>)
    @extends gtk4::Widget;
}*/

// ----- begin of macro expansion of glib::wrapper -----
#[repr(transparent)]
/// `DataType`: This is the type which should be stored.
///
/// `TYPE_NAME`: This is the name by which this type is registered in the glib. Should be: "FixIndexedList_DataType" where "DataType" is to be replaced by the name of `DataType`. Has to be unique in the whole application.
///
/// `ENTRY_TYPE_NAME`: This is the name by which the entry type is registered in the glib. Should be: "FixIndexedListEntry_DataType" where "DataType" is to be replaced by the name of `DataType`. Has to be unique in the whole application.
pub struct FixIndexedList<
    DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static,
    const TYPE_NAME: &'static str,
    const ENTRY_TYPE_NAME: &'static str,
> {
    inner: glib::object::TypedObjectRef<
        inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>,
        <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::ParentType,
    >,
    phantom: std::marker::PhantomData<DataType>,
}
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    std::clone::Clone for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: std::clone::Clone::clone(&self.inner),
            phantom: std::marker::PhantomData,
        }
    }
}
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    std::hash::Hash for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        std::hash::Hash::hash(&self.inner, state);
    }
}
impl<
        OT: glib::object::ObjectType,
        DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static,
        const TYPE_NAME: &'static str,
        const ENTRY_TYPE_NAME: &'static str,
    > std::cmp::PartialEq<OT> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn eq(&self, other: &OT) -> bool {
        std::cmp::PartialEq::eq(&*self.inner, glib::object::ObjectType::as_object_ref(other))
    }
}
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    std::cmp::Eq for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
}
impl<
        OT: glib::object::ObjectType,
        DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static,
        const TYPE_NAME: &'static str,
        const ENTRY_TYPE_NAME: &'static str,
    > std::cmp::PartialOrd<OT> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn partial_cmp(&self, other: &OT) -> Option<std::cmp::Ordering> {
        std::cmp::PartialOrd::partial_cmp(&*self.inner, glib::object::ObjectType::as_object_ref(other))
    }
}
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    std::cmp::Ord for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ord::cmp(&*self.inner, glib::object::ObjectType::as_object_ref(other))
    }
}
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    std::fmt::Debug for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FixIndexedList").field("inner", &self.inner).finish()
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    From<FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>> for glib::object::ObjectRef
{
    #[inline]
    fn from(s: FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>) -> glib::object::ObjectRef {
        s.inner.into_inner()
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::UnsafeFrom<glib::object::ObjectRef> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    unsafe fn unsafe_from(t: glib::object::ObjectRef) -> Self {
        FixIndexedList {
            inner: glib::object::TypedObjectRef::new(t),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::GlibPtrDefault for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type GlibType = *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance;
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::TransparentPtrType for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::object::ObjectType for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type GlibType = <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance;
    type GlibClassType = <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Class;
    #[inline]
    fn as_object_ref(&self) -> &glib::object::ObjectRef {
        &self.inner
    }
    #[inline]
    fn as_ptr(&self) -> *mut Self::GlibType {
        unsafe {
            *(self as *const Self
                as *const *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance)
                as *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance
        }
    }
    #[inline]
    unsafe fn from_glib_ptr_borrow<'a>(ptr: *const *const Self::GlibType) -> &'a Self {
        &*(ptr as *const Self)
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    AsRef<glib::object::ObjectRef> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn as_ref(&self) -> &glib::object::ObjectRef {
        &self.inner
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    AsRef<Self> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::object::IsA<Self> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::subclass::types::FromObject for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type FromObjectType = Self;
    #[inline]
    fn from_object(obj: &Self::FromObjectType) -> &Self {
        obj
    }
}
#[doc(hidden)]
impl<'a, DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::ToGlibPtr<
        'a,
        *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Storage = <glib::object::ObjectRef as glib::translate::ToGlibPtr<'a, *mut glib::gobject_ffi::GObject>>::Storage;
    #[inline]
    fn to_glib_none(
        &'a self,
    ) -> glib::translate::Stash<
        'a,
        *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self,
    > {
        let stash = glib::translate::ToGlibPtr::to_glib_none(&*self.inner);
        glib::translate::Stash(stash.0 as *const _, stash.1)
    }
    #[inline]
    fn to_glib_full(
        &self,
    ) -> *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        glib::translate::ToGlibPtr::to_glib_full(&*self.inner) as *const _
    }
}
#[doc(hidden)]
impl<'a, DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::ToGlibPtr<
        'a,
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Storage = <glib::object::ObjectRef as glib::translate::ToGlibPtr<'a, *mut glib::gobject_ffi::GObject>>::Storage;
    #[inline]
    fn to_glib_none(
        &'a self,
    ) -> glib::translate::Stash<
        'a,
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self,
    > {
        let stash = glib::translate::ToGlibPtr::to_glib_none(&*self.inner);
        glib::translate::Stash(stash.0 as *mut _, stash.1)
    }
    #[inline]
    fn to_glib_full(&self) -> *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        glib::translate::ToGlibPtr::to_glib_full(&*self.inner) as *mut _
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::IntoGlibPtr<
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    unsafe fn into_glib_ptr(
        self,
    ) -> *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        let s = std::mem::ManuallyDrop::new(self);
        glib::translate::ToGlibPtr::<
            *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        >::to_glib_none(&*s)
        .0 as *mut _
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::IntoGlibPtr<
        *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    unsafe fn into_glib_ptr(
        self,
    ) -> *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        let s = std::mem::ManuallyDrop::new(self);
        glib::translate::ToGlibPtr::<
            *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        >::to_glib_none(&*s)
        .0 as *const _
    }
}
#[doc(hidden)]
impl<'a, DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::ToGlibContainerFromSlice<
        'a,
        *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Storage = (
        std::marker::PhantomData<&'a [Self]>,
        Option<Vec<*mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>>,
    );
    fn to_glib_none_from_slice(
        t: &'a [Self],
    ) -> (
        *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        let mut v_ptr = Vec::with_capacity(t.len() + 1);
        unsafe {
            let ptr = v_ptr.as_mut_ptr();
            std::ptr::copy_nonoverlapping(
                t.as_ptr()
                    as *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
                ptr,
                t.len(),
            );
            std::ptr::write(ptr.add(t.len()), std::ptr::null_mut());
            v_ptr.set_len(t.len() + 1);
        }
        (
            v_ptr.as_ptr()
                as *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
            (std::marker::PhantomData, Some(v_ptr)),
        )
    }
    fn to_glib_container_from_slice(
        t: &'a [Self],
    ) -> (
        *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        let v_ptr = unsafe {
            let v_ptr = glib::ffi::g_malloc(
                std::mem::size_of::<
                    *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
                >() * (t.len() + 1),
            )
                as *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance;
            std::ptr::copy_nonoverlapping(
                t.as_ptr()
                    as *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
                v_ptr,
                t.len(),
            );
            std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());
            v_ptr
        };
        (v_ptr, (std::marker::PhantomData, None))
    }
    fn to_glib_full_from_slice(
        t: &[Self],
    ) -> *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        unsafe {
            let v_ptr = glib::ffi::g_malloc(
                std::mem::size_of::<
                    *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
                >() * (t.len() + 1),
            )
                as *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance;
            for (i, s) in t.iter().enumerate() {
                std::ptr::write(v_ptr.add(i), glib::translate::ToGlibPtr::to_glib_full(s));
            }
            std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());
            v_ptr
        }
    }
}
#[doc(hidden)]
impl<'a, DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::ToGlibContainerFromSlice<
        'a,
        *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Storage = (
        std::marker::PhantomData<&'a [Self]>,
        Option<Vec<*mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>>,
    );
    fn to_glib_none_from_slice(
        t: &'a [Self],
    ) -> (
        *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        let (ptr, stash) = glib::translate::ToGlibContainerFromSlice::<
            'a,
            *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        >::to_glib_none_from_slice(t);
        (
            ptr as *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
            stash,
        )
    }
    fn to_glib_container_from_slice(
        _: &'a [Self],
    ) -> (
        *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        panic!("not implemented")
    }
    fn to_glib_full_from_slice(
        _: &[Self],
    ) -> *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        panic!("not implemented")
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrNone<
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_none(
        ptr: *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert!(glib::types::instance_of::<Self>(ptr as *const _));
        FixIndexedList {
            inner: glib::object::TypedObjectRef::new(glib::translate::from_glib_none(ptr as *mut _)),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrNone<
        *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_none(
        ptr: *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert!(glib::types::instance_of::<Self>(ptr as *const _));
        FixIndexedList {
            inner: glib::object::TypedObjectRef::new(glib::translate::from_glib_none(ptr as *mut _)),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrFull<
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_full(
        ptr: *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert!(glib::types::instance_of::<Self>(ptr as *const _));
        FixIndexedList {
            inner: glib::object::TypedObjectRef::new(glib::translate::from_glib_full(ptr as *mut _)),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrBorrow<
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_borrow(
        ptr: *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> glib::translate::Borrowed<Self> {
        debug_assert!(!ptr.is_null());
        debug_assert!(glib::types::instance_of::<Self>(ptr as *const _));
        glib::translate::Borrowed::new(FixIndexedList {
            inner: glib::object::TypedObjectRef::new(glib::translate::from_glib_borrow::<_, glib::object::ObjectRef>(ptr as *mut _).into_inner()),
            phantom: std::marker::PhantomData,
        })
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrBorrow<
        *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_borrow(
        ptr: *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::from_glib_borrow::<_, Self>(
            ptr as *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        )
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::FromGlibContainerAsVec<
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    unsafe fn from_glib_none_num_as_vec(
        ptr: *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let mut res = Vec::<Self>::with_capacity(num);
        let res_ptr = res.as_mut_ptr();
        for i in 0..num {
            ::std::ptr::write(res_ptr.add(i), glib::translate::from_glib_none(std::ptr::read(ptr.add(i))));
        }
        res.set_len(num);
        res
    }
    unsafe fn from_glib_container_num_as_vec(
        ptr: *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        let res = glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }
    unsafe fn from_glib_full_num_as_vec(
        ptr: *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            glib::ffi::g_free(ptr as *mut _);
            return Vec::new();
        }
        let mut res = Vec::with_capacity(num);
        let res_ptr = res.as_mut_ptr();
        ::std::ptr::copy_nonoverlapping(ptr as *mut Self, res_ptr, num);
        res.set_len(num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrArrayContainerAsVec<
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    unsafe fn from_glib_none_as_vec(
        ptr: *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
    }
    unsafe fn from_glib_container_as_vec(
        ptr: *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        glib::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
    }
    unsafe fn from_glib_full_as_vec(
        ptr: *mut *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        glib::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::FromGlibContainerAsVec<
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    unsafe fn from_glib_none_num_as_vec(
        ptr: *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
    }
    unsafe fn from_glib_container_num_as_vec(
        _: *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        _: usize,
    ) -> Vec<Self> {
        panic!("not implemented")
    }
    unsafe fn from_glib_full_num_as_vec(
        _: *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        _: usize,
    ) -> Vec<Self> {
        panic!("not implemented")
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrArrayContainerAsVec<
        *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    unsafe fn from_glib_none_as_vec(
        ptr: *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        glib::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
    }
    unsafe fn from_glib_container_as_vec(
        _: *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        panic!("not implemented")
    }
    unsafe fn from_glib_full_as_vec(
        _: *const *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        panic!("not implemented")
    }
}
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::types::StaticType for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn static_type() -> glib::types::Type {
        #[allow(unused_unsafe)]
        unsafe {
            glib::translate::from_glib(glib::translate::IntoGlib::into_glib(<inner::FixIndexedList<
                DataType,
                TYPE_NAME,
                ENTRY_TYPE_NAME,
            > as glib::subclass::types::ObjectSubclassType>::type_(
            )))
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::value::ValueType for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Type = FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>;
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::value::ValueTypeOptional for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
}
#[doc(hidden)]
unsafe impl<'a, DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::value::FromValue<'a> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Checker = glib::object::ObjectValueTypeChecker<Self>;
    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        let ptr = glib::gobject_ffi::g_value_dup_object(glib::translate::ToGlibPtr::to_glib_none(value).0);
        debug_assert!(!ptr.is_null());
        debug_assert_ne!((*ptr).ref_count, 0);
        <Self as glib::translate::FromGlibPtrFull<
            *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        >>::from_glib_full(
            ptr as *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        )
    }
}
#[doc(hidden)]
unsafe impl<'a, DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::value::FromValue<'a> for &'a FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Checker = glib::object::ObjectValueTypeChecker<Self>;
    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        debug_assert_eq!(std::mem::size_of::<Self>(), std::mem::size_of::<glib::ffi::gpointer>());
        let value = &*(value as *const glib::Value as *const glib::gobject_ffi::GValue);
        debug_assert!(!value.data[0].v_pointer.is_null());
        debug_assert_ne!((*(value.data[0].v_pointer as *const glib::gobject_ffi::GObject)).ref_count, 0);

        <FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::object::ObjectType>::from_glib_ptr_borrow(
            &value.data[0].v_pointer as *const glib::ffi::gpointer
                as *const *const <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        )
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::value::ToValue for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn to_value(&self) -> glib::Value {
        unsafe {
            let mut value = glib::Value::from_type_unchecked(<Self as glib::StaticType>::static_type());
            glib::gobject_ffi::g_value_take_object(
                glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                glib::translate::ToGlibPtr::<
                    *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
                >::to_glib_full(self) as *mut _,
            );
            value
        }
    }
    #[inline]
    fn value_type(&self) -> glib::Type {
        <Self as glib::StaticType>::static_type()
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    ::std::convert::From<FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>> for glib::Value
{
    #[inline]
    fn from(o: FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>) -> Self {
        unsafe {
            let mut value =
                glib::Value::from_type_unchecked(<FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::StaticType>::static_type());
            glib::gobject_ffi::g_value_take_object(
                glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                glib::translate::IntoGlibPtr::<
                    *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
                >::into_glib_ptr(o) as *mut _,
            );
            value
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::value::ToValueOptional for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_take_object(
                glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                glib::translate::ToGlibPtr::<
                    *mut <inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
                >::to_glib_full(&s) as *mut _,
            );
        }
        value
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::clone::Downgrade for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Weak = glib::object::WeakRef<Self>;
    #[inline]
    fn downgrade(&self) -> Self::Weak {
        <Self as glib::object::ObjectExt>::downgrade(&self)
    }
}
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::HasParamSpec for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type ParamSpec = glib::ParamSpecObject;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecObjectBuilder<Self>;
    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}
unsafe impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::object::IsA<gtk4::Widget> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    From<FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>> for gtk4::Widget
{
    #[inline]
    fn from(v: FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>) -> Self {
        <FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::Cast>::upcast(v)
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    AsRef<gtk4::Widget> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn as_ref(&self) -> &gtk4::Widget {
        glib::object::Cast::upcast_ref(self)
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    std::borrow::Borrow<gtk4::Widget> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn borrow(&self) -> &gtk4::Widget {
        glib::object::Cast::upcast_ref(self)
    }
}
unsafe impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::object::ParentClassIs for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Parent = gtk4::Widget;
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    AsRef<glib::object::Object> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn as_ref(&self) -> &glib::object::Object {
        glib::object::Cast::upcast_ref(self)
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    std::borrow::Borrow<glib::object::Object> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    #[inline]
    fn borrow(&self) -> &glib::object::Object {
        glib::object::Cast::upcast_ref(self)
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    From<FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>> for glib::object::Object
{
    #[inline]
    fn from(v: FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>) -> Self {
        <FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME> as glib::Cast>::upcast(v)
    }
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::object::IsA<glib::object::Object> for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::object::IsClass for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
}
unsafe impl<DataType: Default + ObjectExt + IsA<Object> + Into<Value> + 'static, const TYPE_NAME: &'static str, const ENTRY_TYPE_NAME: &'static str>
    glib::object::ObjectSubclassIs for FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>
{
    type Subclass = inner::FixIndexedList<DataType, TYPE_NAME, ENTRY_TYPE_NAME>;
}
// ----- end of macro expansion of glib::wrapper -----
