use std::{
  sync::{Mutex, Arc},
  collections::HashMap
};

pub struct DataItem <T> {
  data: Mutex<T>,
}

pub type Store = HashMap<String, Arc<DataItem<String>>>;

// Thread safe method to get a value
pub fn get_value_safe(data: &Store, key: String) -> String {
  if let Some(d) = data.get(&key) {
    // Lock mutex
    let store_data = d.data.lock().unwrap();
    // Return value
    return store_data.clone();
  } else {
    // No value
    return String::new();
  }
}

// Thread safe method to save a value
pub fn set_value_safe(data: &mut Store, key: String, insert_data: Vec<String>) {
  if let Some(d) = data.get(&key) {
    // Key already exists--lock mutex & update it
    let mut store_data = d.data.lock().unwrap();
    *store_data = insert_data.join(" ");
  } else {
    // Key does not exist--insert it
    let data_to_insert = Arc::new(DataItem {
      data: Mutex::new(insert_data.join(" "))
    });

    data.insert(key, Arc::clone(&data_to_insert));
  }
}

pub fn delete_value_safe(data: &mut Store, key: String) -> Option<()> {
  data.remove(&key)?;
  Some(())
}