use std::{
  sync::{Mutex, Arc, RwLock},
  collections::HashMap
};

pub struct DataItem <T> {
  data: Mutex<T>,
}

pub type Store = Arc<RwLock<HashMap<String, Arc<DataItem<String>>>>>;

// Thread safe method to get a value
pub fn get_value_safe(store: &Store, key: String) -> String {
  let data = store.read().unwrap();
  return if let Some(d) = data.get(&key) {
    // Lock mutex
    let store_data = d.data.lock().unwrap();
    // Return value
    store_data.clone()
  } else {
    // No value
    String::new()
  }
}

// Thread safe method to save a value
pub fn set_value_safe(store: &mut Store, key: String, insert_data: Vec<String>) {
  let mut data = store.write().unwrap();
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

pub fn delete_value_safe(store: &mut Store, key: String) -> bool {
  let mut data = store.write().unwrap();
  if data.contains_key(&key) {
    data.remove(&key).is_some()
  } else {
    false
  }
  // if let Some(d) = data.get(&key) {
  //   // Lock mutex then delete
  //   let _delete_data = d.data.lock().unwrap();
  //   data.remove(&key);
  //   return true
  // } else {
  //   return false
  // }
}

// Unit Tests
#[cfg(test)]
mod tests {
  use super::*;

  struct TestItem {
    key: String,
    value: String,
  }

  fn seed_store_with_values(values: Vec<TestItem>) -> Store {
    let store: Store = Arc::new(RwLock::new(HashMap::new()));
    {
      let mut data = store.write().unwrap();

      for value in values {
        data.insert(value.key.to_string(), Arc::new(DataItem {
          data: Mutex::new(value.value)
        }));
      }
    }

    store
  }

  #[test]
  fn get_value_safe_invalid_key() {
    let store: Store = seed_store_with_values(vec![
      TestItem { key: String::from("alpha"), value: String::from("hello there") },
      TestItem { key: String::from("bravo"), value: String::from("good morning") },
    ]);

    let result: String = get_value_safe(&store, String::from("charlie"));

    assert_eq!(result, String::new())
  }

  #[test]
  fn get_value_safe_valid_key() {
    let store: Store = seed_store_with_values(vec![
      TestItem { key: String::from("alpha"), value: String::from("hello there") },
      TestItem { key: String::from("bravo"), value: String::from("good morning") },
      TestItem { key: String::from("charlie"), value: String::from("whats up?") },
    ]);

    let result: String = get_value_safe(&store, String::from("charlie"));

    assert_eq!(result, String::from("whats up?"))
  }

  #[test]
  fn set_value_safe_valid() {
    let mut store: Store = Arc::new(RwLock::new(HashMap::new()));
    {
      set_value_safe(&mut store, String::from("alpha"), vec![String::from("Hello"), String::from("World")]);
    }
    let data = store.read().unwrap();
    let mut result = String::new();
    // let result: String = get_value_safe(&store, String::from("alpha"));
    if let Some(d) = data.get(&String::from("alpha")) {
      // Lock mutex
      let store_data = d.data.lock().unwrap();
      // Return value
      result = store_data.clone();
    }

    assert_eq!(result, String::from("Hello World"));
  }

  #[test]
  fn delete_value_safe_valid_key() {
    let mut store: Store = seed_store_with_values(vec![
      TestItem { key: String::from("alpha"), value: String::from("hello there") },
      TestItem { key: String::from("bravo"), value: String::from("good morning") },
      TestItem { key: String::from("charlie"), value: String::from("whats up?") },
    ]);
    {
      let delete_result: bool = delete_value_safe(&mut store, String::from("alpha"));
      assert!(delete_result);
    }

    let data = store.read().unwrap();
    let mut result = String::new();
    // let result: String = get_value_safe(&store, String::from("alpha"));
    if let Some(d) = data.get(&String::from("alpha")) {
      // Lock mutex
      let store_data = d.data.lock().unwrap();
      // Return value
      result = store_data.clone();
    }
    assert_eq!(result, String::new());
  }

}
