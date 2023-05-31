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
    pub struct FixIndexedListEntry<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> {
        pub position: Cell<u32>,
        pub data: RefCell<DataType>,
    }

    // ----- begin of macro expansion of glib::object_subclass -----
    unsafe impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> ObjectSubclassType for FixIndexedListEntry<DataType, TYPE_NAME> {
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
    impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> FromObject for FixIndexedListEntry<DataType, TYPE_NAME> {
        type FromObjectType = <Self as ObjectSubclass>::Type;
        #[inline]
        fn from_object(obj: &Self::FromObjectType) -> &Self {
            <Self as ObjectSubclassExt>::from_obj(obj)
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> Downgrade for FixIndexedListEntry<DataType, TYPE_NAME> {
        type Weak = ObjectImplWeakRef<FixIndexedListEntry<DataType, TYPE_NAME>>;

        #[inline]
        fn downgrade(&self) -> Self::Weak {
            let ref_counted = ObjectSubclassExt::ref_counted(self);
            Downgrade::downgrade(&ref_counted)
        }
    }

    impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> FixIndexedListEntry<DataType, TYPE_NAME> {
        #[inline]
        pub fn downgrade(&self) -> <Self as Downgrade>::Weak {
            Downgrade::downgrade(self)
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> ::std::borrow::ToOwned for FixIndexedListEntry<DataType, TYPE_NAME> {
        type Owned = ObjectImplRef<FixIndexedListEntry<DataType, TYPE_NAME>>;

        #[inline]
        fn to_owned(&self) -> Self::Owned {
            ObjectSubclassExt::ref_counted(self)
        }
    }

    #[doc(hidden)]
    impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> ::std::borrow::Borrow<FixIndexedListEntry<DataType, TYPE_NAME>>
        for ObjectImplRef<FixIndexedListEntry<DataType, TYPE_NAME>>
    {
        #[inline]
        fn borrow(&self) -> &FixIndexedListEntry<DataType, TYPE_NAME> {
            self
        }
    }

    impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> ObjectSubclass for FixIndexedListEntry<DataType, TYPE_NAME> {
        const NAME: &'static str = TYPE_NAME;
        type Type = super::FixIndexedListEntry<DataType, TYPE_NAME>;
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

    impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> ObjectImpl for FixIndexedListEntry<DataType, TYPE_NAME> {}
}

// The implementation is above the struct definition in order to keep the actual important part visible and not hidden in the mess below.
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> FixIndexedListEntry<DataType, TYPE_NAME> {
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

// This mess below is the expansion of the following macro, but as this does not work out of the box, we expanded it manually.
// Caution: Can break with newer versions of this macro.
/*glib::wrapper! {
    pub struct FixIndexedListEntry<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>(ObjectSubclass<inner::FixIndexedListEntry<DataType, Type_NAME>>);
}*/

// ----- begin of macro expansion of glib::wrapper -----
#[repr(transparent)]
///
/// `DataType`: This is the type which should be stored.
///
/// `TYPE_NAME`: This is the name by which this type is registered in the glib. Should be: "FixIndexedListEntry_DataType" where "DataType" has to be replaced by the name of `DataType`. Has to be unique in the whole application.
///
pub struct FixIndexedListEntry<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> {
    inner: glib::object::TypedObjectRef<inner::FixIndexedListEntry<DataType, TYPE_NAME>, ()>,
    phantom: std::marker::PhantomData<DataType>,
}
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> std::clone::Clone for FixIndexedListEntry<DataType, TYPE_NAME> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: std::clone::Clone::clone(&self.inner),
            phantom: std::marker::PhantomData,
        }
    }
}
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> std::hash::Hash for FixIndexedListEntry<DataType, TYPE_NAME> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        std::hash::Hash::hash(&self.inner, state);
    }
}
impl<OT: glib::object::ObjectType, DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> std::cmp::PartialEq<OT>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    fn eq(&self, other: &OT) -> bool {
        std::cmp::PartialEq::eq(&*self.inner, glib::object::ObjectType::as_object_ref(other))
    }
}
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> std::cmp::Eq for FixIndexedListEntry<DataType, TYPE_NAME> {}
impl<OT: glib::object::ObjectType, DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> std::cmp::PartialOrd<OT>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    fn partial_cmp(&self, other: &OT) -> Option<std::cmp::Ordering> {
        std::cmp::PartialOrd::partial_cmp(&*self.inner, glib::object::ObjectType::as_object_ref(other))
    }
}
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> std::cmp::Ord for FixIndexedListEntry<DataType, TYPE_NAME> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ord::cmp(&*self.inner, glib::object::ObjectType::as_object_ref(other))
    }
}
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> std::fmt::Debug for FixIndexedListEntry<DataType, TYPE_NAME> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FixIndexedListEntry").field("inner", &self.inner).finish()
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> From<FixIndexedListEntry<DataType, TYPE_NAME>>
    for glib::object::ObjectRef
{
    #[inline]
    fn from(s: FixIndexedListEntry<DataType, TYPE_NAME>) -> glib::object::ObjectRef {
        s.inner.into_inner()
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::translate::UnsafeFrom<glib::object::ObjectRef>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    unsafe fn unsafe_from(t: glib::object::ObjectRef) -> Self {
        FixIndexedListEntry {
            inner: glib::object::TypedObjectRef::new(t),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::translate::GlibPtrDefault
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type GlibType = *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance;
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::translate::TransparentPtrType
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::object::ObjectType
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type GlibType = <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance;
    type GlibClassType = <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Class;
    #[inline]
    fn as_object_ref(&self) -> &glib::object::ObjectRef {
        &self.inner
    }
    #[inline]
    fn as_ptr(&self) -> *mut Self::GlibType {
        unsafe {
            *(self as *const Self
                as *const *const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance)
                as *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance
        }
    }
    #[inline]
    unsafe fn from_glib_ptr_borrow<'a>(ptr: *const *const Self::GlibType) -> &'a Self {
        &*(ptr as *const Self)
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> AsRef<glib::object::ObjectRef>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    fn as_ref(&self) -> &glib::object::ObjectRef {
        &self.inner
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> AsRef<Self> for FixIndexedListEntry<DataType, TYPE_NAME> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::object::IsA<Self>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::subclass::types::FromObject
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type FromObjectType = Self;
    #[inline]
    fn from_object(obj: &Self::FromObjectType) -> &Self {
        obj
    }
}
#[doc(hidden)]
impl<'a, DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::ToGlibPtr<'a, *const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type Storage = <glib::object::ObjectRef as glib::translate::ToGlibPtr<'a, *mut glib::gobject_ffi::GObject>>::Storage;
    #[inline]
    fn to_glib_none(
        &'a self,
    ) -> glib::translate::Stash<'a, *const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance, Self>
    {
        let stash = glib::translate::ToGlibPtr::to_glib_none(&*self.inner);
        glib::translate::Stash(stash.0 as *const _, stash.1)
    }
    #[inline]
    fn to_glib_full(&self) -> *const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        glib::translate::ToGlibPtr::to_glib_full(&*self.inner) as *const _
    }
}
#[doc(hidden)]
impl<'a, DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::ToGlibPtr<'a, *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type Storage = <glib::object::ObjectRef as glib::translate::ToGlibPtr<'a, *mut glib::gobject_ffi::GObject>>::Storage;
    #[inline]
    fn to_glib_none(
        &'a self,
    ) -> glib::translate::Stash<'a, *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance, Self>
    {
        let stash = glib::translate::ToGlibPtr::to_glib_none(&*self.inner);
        glib::translate::Stash(stash.0 as *mut _, stash.1)
    }
    #[inline]
    fn to_glib_full(&self) -> *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        glib::translate::ToGlibPtr::to_glib_full(&*self.inner) as *mut _
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::IntoGlibPtr<*mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    unsafe fn into_glib_ptr(self) -> *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        let s = std::mem::ManuallyDrop::new(self);
        glib::translate::ToGlibPtr::<
                *const <inner::FixIndexedListEntry<
                    DataType, TYPE_NAME
                > as glib::subclass::types::ObjectSubclass>::Instance,
            >::to_glib_none(&*s)
                .0 as *mut _
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::IntoGlibPtr<*const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    unsafe fn into_glib_ptr(self) -> *const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        let s = std::mem::ManuallyDrop::new(self);
        glib::translate::ToGlibPtr::<
                *const <inner::FixIndexedListEntry<
                    DataType, TYPE_NAME
                > as glib::subclass::types::ObjectSubclass>::Instance,
            >::to_glib_none(&*s)
                .0 as *const _
    }
}
#[doc(hidden)]
impl<'a, DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::ToGlibContainerFromSlice<
        'a,
        *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type Storage = (
        std::marker::PhantomData<&'a [Self]>,
        Option<Vec<*mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>>,
    );
    fn to_glib_none_from_slice(
        t: &'a [Self],
    ) -> (
        *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        let mut v_ptr = Vec::with_capacity(t.len() + 1);
        unsafe {
            let ptr = v_ptr.as_mut_ptr();
            std::ptr::copy_nonoverlapping(
                t.as_ptr() as *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
                ptr,
                t.len(),
            );
            std::ptr::write(ptr.add(t.len()), std::ptr::null_mut());
            v_ptr.set_len(t.len() + 1);
        }
        (
            v_ptr.as_ptr() as *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
            (std::marker::PhantomData, Some(v_ptr)),
        )
    }
    fn to_glib_container_from_slice(
        t: &'a [Self],
    ) -> (
        *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        let v_ptr = unsafe {
            let v_ptr = glib::ffi::g_malloc(
                std::mem::size_of::<*mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>()
                    * (t.len() + 1),
            )
                as *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance;
            std::ptr::copy_nonoverlapping(
                t.as_ptr() as *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
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
    ) -> *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        unsafe {
            let v_ptr = glib::ffi::g_malloc(
                std::mem::size_of::<*mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>()
                    * (t.len() + 1),
            )
                as *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance;
            for (i, s) in t.iter().enumerate() {
                std::ptr::write(v_ptr.add(i), glib::translate::ToGlibPtr::to_glib_full(s));
            }
            std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());
            v_ptr
        }
    }
}
#[doc(hidden)]
impl<'a, DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::ToGlibContainerFromSlice<
        'a,
        *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type Storage = (
        std::marker::PhantomData<&'a [Self]>,
        Option<Vec<*mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>>,
    );
    fn to_glib_none_from_slice(
        t: &'a [Self],
    ) -> (
        *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        let (ptr, stash) = glib::translate::ToGlibContainerFromSlice::<
            'a,
            *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        >::to_glib_none_from_slice(t);
        (
            ptr as *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
            stash,
        )
    }
    fn to_glib_container_from_slice(
        _: &'a [Self],
    ) -> (
        *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        panic!("not implemented")
    }
    fn to_glib_full_from_slice(
        _: &[Self],
    ) -> *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance {
        panic!("not implemented")
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrNone<*mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_none(ptr: *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert!(glib::types::instance_of::<Self>(ptr as *const _));
        FixIndexedListEntry {
            inner: glib::object::TypedObjectRef::new(glib::translate::from_glib_none(ptr as *mut _)),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrNone<*const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_none(
        ptr: *const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert!(glib::types::instance_of::<Self>(ptr as *const _));
        FixIndexedListEntry {
            inner: glib::object::TypedObjectRef::new(glib::translate::from_glib_none(ptr as *mut _)),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrFull<*mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_full(ptr: *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert!(glib::types::instance_of::<Self>(ptr as *const _));
        FixIndexedListEntry {
            inner: glib::object::TypedObjectRef::new(glib::translate::from_glib_full(ptr as *mut _)),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrBorrow<*mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_borrow(
        ptr: *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> glib::translate::Borrowed<Self> {
        debug_assert!(!ptr.is_null());
        debug_assert!(glib::types::instance_of::<Self>(ptr as *const _));
        glib::translate::Borrowed::new(FixIndexedListEntry {
            inner: glib::object::TypedObjectRef::new(glib::translate::from_glib_borrow::<_, glib::object::ObjectRef>(ptr as *mut _).into_inner()),
            phantom: std::marker::PhantomData,
        })
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrBorrow<*const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_borrow(
        ptr: *const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::from_glib_borrow::<_, Self>(
            ptr as *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        )
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::FromGlibContainerAsVec<
        *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedListEntry<DataType, TYPE_NAME>
{
    unsafe fn from_glib_none_num_as_vec(
        ptr: *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
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
        ptr: *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        let res = glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }
    unsafe fn from_glib_full_num_as_vec(
        ptr: *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
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
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrArrayContainerAsVec<
        *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedListEntry<DataType, TYPE_NAME>
{
    unsafe fn from_glib_none_as_vec(
        ptr: *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
    }
    unsafe fn from_glib_container_as_vec(
        ptr: *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        glib::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
    }
    unsafe fn from_glib_full_as_vec(
        ptr: *mut *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        glib::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::FromGlibContainerAsVec<
        *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedListEntry<DataType, TYPE_NAME>
{
    unsafe fn from_glib_none_num_as_vec(
        ptr: *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
    }
    unsafe fn from_glib_container_num_as_vec(
        _: *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        _: usize,
    ) -> Vec<Self> {
        panic!("not implemented")
    }
    unsafe fn from_glib_full_num_as_vec(
        _: *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        _: usize,
    ) -> Vec<Self> {
        panic!("not implemented")
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str>
    glib::translate::FromGlibPtrArrayContainerAsVec<
        *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    > for FixIndexedListEntry<DataType, TYPE_NAME>
{
    unsafe fn from_glib_none_as_vec(
        ptr: *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        glib::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
    }
    unsafe fn from_glib_container_as_vec(
        _: *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        panic!("not implemented")
    }
    unsafe fn from_glib_full_as_vec(
        _: *const *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        panic!("not implemented")
    }
}
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::types::StaticType for FixIndexedListEntry<DataType, TYPE_NAME> {
    #[inline]
    fn static_type() -> glib::types::Type {
        #[allow(unused_unsafe)]
        unsafe {
            glib::translate::from_glib(glib::translate::IntoGlib::into_glib(
                <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclassType>::type_(),
            ))
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::value::ValueType for FixIndexedListEntry<DataType, TYPE_NAME> {
    type Type = FixIndexedListEntry<DataType, TYPE_NAME>;
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::value::ValueTypeOptional
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
}
#[doc(hidden)]
unsafe impl<'a, DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::value::FromValue<'a>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type Checker = glib::object::ObjectValueTypeChecker<Self>;
    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        let ptr = glib::gobject_ffi::g_value_dup_object(glib::translate::ToGlibPtr::to_glib_none(value).0);
        debug_assert!(!ptr.is_null());
        debug_assert_ne!((*ptr).ref_count, 0);
        <Self as glib::translate::FromGlibPtrFull<
            *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        >>::from_glib_full(ptr as *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance)
    }
}
#[doc(hidden)]
unsafe impl<'a, DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::value::FromValue<'a>
    for &'a FixIndexedListEntry<DataType, TYPE_NAME>
{
    type Checker = glib::object::ObjectValueTypeChecker<Self>;
    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        debug_assert_eq!(std::mem::size_of::<Self>(), std::mem::size_of::<glib::ffi::gpointer>());
        let value = &*(value as *const glib::Value as *const glib::gobject_ffi::GValue);
        debug_assert!(!value.data[0].v_pointer.is_null());
        debug_assert_ne!((*(value.data[0].v_pointer as *const glib::gobject_ffi::GObject)).ref_count, 0);
        <FixIndexedListEntry<DataType, TYPE_NAME> as glib::object::ObjectType>::from_glib_ptr_borrow(
            &value.data[0].v_pointer as *const glib::ffi::gpointer
                as *const *const <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
        )
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::value::ToValue for FixIndexedListEntry<DataType, TYPE_NAME> {
    #[inline]
    fn to_value(&self) -> glib::Value {
        unsafe {
            let mut value = glib::Value::from_type_unchecked(<Self as glib::StaticType>::static_type());
            glib::gobject_ffi::g_value_take_object(
                    glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                    glib::translate::ToGlibPtr::<
                        *mut <inner::FixIndexedListEntry<
                            DataType, TYPE_NAME
                        > as glib::subclass::types::ObjectSubclass>::Instance,
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
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> ::std::convert::From<FixIndexedListEntry<DataType, TYPE_NAME>>
    for glib::Value
{
    #[inline]
    fn from(o: FixIndexedListEntry<DataType, TYPE_NAME>) -> Self {
        unsafe {
            let mut value = glib::Value::from_type_unchecked(<FixIndexedListEntry<DataType, TYPE_NAME> as glib::StaticType>::static_type());
            glib::gobject_ffi::g_value_take_object(
                glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                glib::translate::IntoGlibPtr::<
                    *mut <inner::FixIndexedListEntry<DataType, TYPE_NAME> as glib::subclass::types::ObjectSubclass>::Instance,
                >::into_glib_ptr(o) as *mut _,
            );
            value
        }
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::value::ToValueOptional
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_take_object(
                    glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                    glib::translate::ToGlibPtr::<
                        *mut <inner::FixIndexedListEntry<
                            DataType, TYPE_NAME
                        > as glib::subclass::types::ObjectSubclass>::Instance,
                    >::to_glib_full(&s) as *mut _,
                );
        }
        value
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::clone::Downgrade for FixIndexedListEntry<DataType, TYPE_NAME> {
    type Weak = glib::object::WeakRef<Self>;
    #[inline]
    fn downgrade(&self) -> Self::Weak {
        <Self as glib::object::ObjectExt>::downgrade(&self)
    }
}
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::HasParamSpec for FixIndexedListEntry<DataType, TYPE_NAME> {
    type ParamSpec = glib::ParamSpecObject;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecObjectBuilder<Self>;
    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}
unsafe impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::object::ParentClassIs
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type Parent = glib::object::Object;
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> AsRef<glib::object::Object>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    fn as_ref(&self) -> &glib::object::Object {
        glib::object::Cast::upcast_ref(self)
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> std::borrow::Borrow<glib::object::Object>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    #[inline]
    fn borrow(&self) -> &glib::object::Object {
        glib::object::Cast::upcast_ref(self)
    }
}
#[doc(hidden)]
impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> From<FixIndexedListEntry<DataType, TYPE_NAME>> for glib::object::Object {
    #[inline]
    fn from(v: FixIndexedListEntry<DataType, TYPE_NAME>) -> Self {
        <FixIndexedListEntry<DataType, TYPE_NAME> as glib::Cast>::upcast(v)
    }
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::object::IsA<glib::object::Object>
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
}
#[doc(hidden)]
unsafe impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::object::IsClass
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
}
unsafe impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> glib::object::ObjectSubclassIs
    for FixIndexedListEntry<DataType, TYPE_NAME>
{
    type Subclass = inner::FixIndexedListEntry<DataType, TYPE_NAME>;
}
// ----- end of macro expansion of glib::wrapper -----
