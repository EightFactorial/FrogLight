//! TODO
#![allow(missing_docs)]

use core::any::{TypeId, type_name};

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
#[cfg(feature = "std")]
use std::sync::Arc;

use froglight_utils::storage::{AppStorage, IndexedLocalStorage};

fn main() {
    let mut storage = AppMyTraitStorage::from_storage(MyTraitStorage::default());
    // Insert messages into the storage
    storage.write().insert_message(&FirstMessage);
    storage.write().insert_message(&SecondMessage);
    // Print messages from the storage
    storage.read().print_message::<FirstMessage>();
    storage.read().print_message::<SecondMessage>();
    // Print messages from the storage using index
    storage.read().print_message_index(MyIndex::new_unchecked(0));
    storage.read().print_message_index(MyIndex::new_unchecked(1));

    // Create a new storage instance and print messages again
    storage = AppMyTraitStorage::from_storage(MyTraitStorage::default());
    // No messages are stored in the new instance.
    storage.read().print_message::<FirstMessage>();
    storage.read().print_message::<SecondMessage>();
    // No messages are stored in the new instance.
    storage.read().print_message_index(MyIndex::new_unchecked(0));
    storage.read().print_message_index(MyIndex::new_unchecked(1));
}

// -------------------------------------------------------------------------------------------------

#[derive(Default, Clone, AppStorage)]
#[storage(index(ident = "MyIndex", inner = "usize"))]
struct MyTraitStorage(IndexedLocalStorage<dyn MyTrait, MyIndex>);

impl MyTraitStorage {
    fn insert_message<T: MyTrait>(&mut self, message: &'static T) {
        self.0.store(TypeId::of::<T>(), message);
    }

    fn print_message<T: MyTrait>(&self) {
        if let Some(message) = self.0.get(&TypeId::of::<T>()) {
            message.print_message();
        } else {
            println!("No message found for type \"{}\"", type_name::<T>());
        }
    }

    fn print_message_index(&self, index: MyIndex) {
        if let Some(message) = self.0.get_index(index) {
            message.print_message();
        } else {
            println!("No message found for index \"{index:?}\"");
        }
    }
}

// -------------------------------------------------------------------------------------------------

trait MyTrait: 'static {
    fn print_message(&self);
}

struct FirstMessage;
impl MyTrait for FirstMessage {
    fn print_message(&self) {
        println!("This is the first message!");
    }
}

struct SecondMessage;
impl MyTrait for SecondMessage {
    fn print_message(&self) {
        println!("This is the second message!");
    }
}
