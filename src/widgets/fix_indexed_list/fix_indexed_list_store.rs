use super::{fix_indexed_list_entry::FixIndexedListEntry, RotateDirection};
use gdk4::prelude::*;
use gtk4::gio::ListStore;
use std::marker::PhantomData;

/// This is a wrapper around a gio::ListStore
/// Every element consists of an index and the actual user data
/// When reordering the elements, the index will stay at the same positions at all times, only the user data is reordered.
///
/// `DataType`: This is the type which should be stored.
///
/// `TYPE_NAME`: This is the name by which the entry type is registered in the glib. Should be: "FixIndexedListEntry_DataType" where "DataType" has to be replaced by the name of `DataType`. Has to be unique in the whole application.
///
#[derive(Debug)]
pub struct FixIndexedListStore<DataType, const TYPE_NAME: &'static str> {
    pub list_store: ListStore,
    phantom_data: PhantomData<DataType>,
}

impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> FixIndexedListStore<DataType, TYPE_NAME> {
    ///
    /// Rotates the entries in the ListStore for positions in the interval `[src_idx, dst_idx]` in `rot_direction`.
    /// Entries rotated out on either side of the interval are moved in from the other side.
    /// The positions stored in the FixIndexedListEntry stay the same, only the user data is rotated around.
    ///
    pub fn rotate_entries(&self, src_idx: u32, dst_idx: u32, rot_direction: RotateDirection) {
        let src_data: FixIndexedListEntry<DataType, TYPE_NAME> = self
            .list_store
            .item(src_idx)
            .expect("Row data must exists!")
            .downcast()
            .expect("Row data has an unexpected type");

        // set teh initial values
        let mut next_modify_idx = dst_idx as i32;
        let mut buf = src_data.get_data();

        // determine which loop condition and which increment step is used, i.e. whether to count up or down
        let (cmp_f, inc): (fn(&i32, &i32) -> bool, i32) = match rot_direction {
            RotateDirection::Up => (i32::le, 1),
            RotateDirection::Down => (i32::ge, -1),
        };
        //
        while cmp_f(&next_modify_idx, &(src_idx as i32)) {
            let row_data: FixIndexedListEntry<DataType, TYPE_NAME> = self
                .list_store
                .item(next_modify_idx as u32)
                .expect("Row data must exists!")
                .downcast()
                .expect("Row data has an unexpected type");

            // exchange the buffers, the buffer of this row will be handed to the the next row
            let next_buf = row_data.get_data();
            row_data.set_data(buf);
            buf = next_buf;

            // emit the signal that the item at this index was changed so that the ListBox updates the row.
            self.list_store.items_changed(next_modify_idx as u32, 1, 1);

            next_modify_idx = next_modify_idx + inc;
        }
    }

    ///
    /// Appends `data` to the end of the ListStore.
    ///
    pub fn append(&self, data: DataType) {
        let entry = FixIndexedListEntry::<DataType, TYPE_NAME>::new(self.n_items(), data);
        self.list_store.append(&entry);
    }

    ///
    /// Removes the data at `index` and returns true if successful.
    ///
    pub fn remove(&self, index: u32) -> bool {
        if index >= self.n_items() {
            return false;
        }

        // first rotate the row at the end of the list
        self.rotate_entries(index, self.n_items() - 1, RotateDirection::Down);
        // then remove it
        self.list_store.remove(self.n_items() - 1);
        true
    }

    ///
    /// Returns the data at `index`.
    ///
    pub fn get(&self, index: u32) -> Option<DataType> {
        let object = self.list_store.item(index);
        let entry_opt: Option<&FixIndexedListEntry<DataType, TYPE_NAME>> = object.and_downcast_ref();
        if let Some(entry) = entry_opt {
            debug_assert_eq!(entry.get_position(), index);
        }
        entry_opt.map(|e| e.get_data())
    }

    ///
    /// Returns the amount of items stored in the list store.
    ///
    pub fn n_items(&self) -> u32 {
        self.list_store.n_items()
    }

    ///
    /// Get the item at `index`.
    ///
    pub fn item(&self, index: u32) -> Option<DataType> {
        self.list_store
            .item(index)
            .and_downcast_ref()
            .map(|x: &FixIndexedListEntry<DataType, TYPE_NAME>| x.get_data())
    }
}

impl<DataType: Default + ObjectExt + 'static, const TYPE_NAME: &'static str> Default for FixIndexedListStore<DataType, TYPE_NAME> {
    fn default() -> Self {
        Self {
            list_store: ListStore::new::<FixIndexedListEntry::<DataType, TYPE_NAME>>(),
            phantom_data: PhantomData::<DataType>::default(),
        }
    }
}
