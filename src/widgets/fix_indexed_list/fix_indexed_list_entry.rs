use gdk4::glib::{
    clone::Downgrade,
    once_cell::sync::Lazy,
    subclass::{types::FromObject, *},
    Object, Type,
};
use gtk4::{glib, prelude::*, subclass::prelude::*};
use std::cell::{Cell, RefCell};
use std::{collections::HashMap, sync::Once};

mod inner {
    use super::*;

    #[derive(Debug, Default)]
    pub struct FixIndexedListEntry<DataType: Default + ObjectExt + 'static> {
        pub position: Cell<u32>,
        pub data: RefCell<DataType>,
    }

    // ----- begin of macro expansion of glib::object_subclass -----
    unsafe impl<DataType: Default + ObjectExt + 'static> ObjectSubclassType for FixIndexedListEntry<DataType> {
        #[inline]
        fn type_data() -> ::std::ptr::NonNull<TypeData> {
            static mut DATA: TypeData = types::INIT_TYPE_DATA;
            unsafe { ::std::ptr::NonNull::from(&mut DATA) }
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

                type_
            }
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + 'static> FromObject for FixIndexedListEntry<DataType> {
        type FromObjectType = <Self as ObjectSubclass>::Type;
        #[inline]
        fn from_object(obj: &Self::FromObjectType) -> &Self {
            <Self as ObjectSubclassExt>::from_obj(obj)
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + 'static> Downgrade for FixIndexedListEntry<DataType> {
        type Weak = ObjectImplWeakRef<FixIndexedListEntry<DataType>>;

        #[inline]
        fn downgrade(&self) -> Self::Weak {
            let ref_counted = ObjectSubclassExt::ref_counted(self);
            Downgrade::downgrade(&ref_counted)
        }
    }

    impl<DataType: Default + ObjectExt + 'static> FixIndexedListEntry<DataType> {
        #[inline]
        pub fn downgrade(&self) -> <Self as Downgrade>::Weak {
            Downgrade::downgrade(self)
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + 'static> ::std::borrow::ToOwned for FixIndexedListEntry<DataType> {
        type Owned = ObjectImplRef<FixIndexedListEntry<DataType>>;

        #[inline]
        fn to_owned(&self) -> Self::Owned {
            ObjectSubclassExt::ref_counted(self)
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + 'static> ::std::borrow::Borrow<FixIndexedListEntry<DataType>> for ObjectImplRef<FixIndexedListEntry<DataType>> {
        #[inline]
        fn borrow(&self) -> &FixIndexedListEntry<DataType> {
            self
        }
    }

    impl<DataType: Default + ObjectExt + 'static> ObjectSubclass for FixIndexedListEntry<DataType> {
        const NAME: &'static str = "FixIndexedListEntry";
        type Type = super::FixIndexedListEntry<DataType>;
        type ParentType = Object;

        type Instance = glib::subclass::basic::InstanceStruct<Self>;
        type Class = glib::subclass::basic::ClassStruct<Self>;
        type Interfaces = ();

        #[inline]
        fn new() -> Self {
            Default::default()
        }
    }
    // ----- end of macro expansion of glib::object_subclass -----

    impl<DataType: Default + ObjectExt + 'static> ObjectImpl for FixIndexedListEntry<DataType> {}
}

glib::wrapper! {
    pub struct FixIndexedListEntry<DataType: Default + ObjectExt + 'static>(ObjectSubclass<inner::FixIndexedListEntry<DataType>>);
}

impl<DataType: Default + ObjectExt + 'static> FixIndexedListEntry<DataType> {
    pub fn new(position: u32, data: DataType) -> Self {
        let obj = Object::new::<Self>();
        obj.set_position(position);
        obj.set_data(data);
        obj
    }

    pub fn get_position(&self) -> u32 {
        self.imp().position.get()
    }

    pub fn set_position(&self, position: u32) {
        self.imp().position.set(position);
    }

    pub fn get_data(&self) -> DataType {
        self.imp().data.borrow().clone()
    }

    pub fn set_data(&self, data: DataType) {
        *self.imp().data.borrow_mut() = data;
    }
}
