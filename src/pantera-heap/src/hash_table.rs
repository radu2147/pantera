use std::alloc::{alloc, Layout};
use std::ptr;
use crate::bytes::{read_byte, read_bytes, read_pointer, write_byte, write_pointer};
use crate::heap::{HeapManager, Ptr};
use crate::types::Type;
use crate::value::Value;

const TABLE_SIZE: usize = 50;

pub struct HashEntry {
    key: Ptr,
    typ: Type,
    value: u64
}

pub unsafe fn get_key(entry: Ptr) -> Ptr {
    read_pointer(entry)
}

pub unsafe fn set_key(entry: Ptr, key: Ptr) {
    write_pointer(entry, key)
}

pub unsafe fn get_type(entry: Ptr) -> Type {
    Type::from(read_byte(entry.add(8)))
}

pub unsafe fn set_type(entry: Ptr, typ: Type) {
    write_byte(entry.add(8), typ.into());
}

pub unsafe fn get_value(entry: Ptr) -> Option<Value> {
    let typ = get_type(entry);
    if matches!(typ, Type::Empty) {
        return None;
    }

    let bytes = read_bytes(entry.add(8 + 1), 8);

    let val = HeapManager::get_raw_value(bytes, get_type(entry));
    Some(val)
}

pub unsafe fn set_value(entry: Ptr, value: Value) {
    HeapManager::allocate_value(&value, entry.add(8));
}


pub struct HashTable {
    entries: *mut u8,
    count: usize
}

impl HashTable {
    pub unsafe fn new() -> Self {
        let layout = Layout::array::<u8>(TABLE_SIZE * Self::size_of()).unwrap();
        let obj_ptr = alloc(layout);
        Self {
            entries: obj_ptr,
            count: 0,
        }
    }

    fn size_of() -> usize {
        8 + 1 + 8
    }

    fn find_entry(&self, key: &Ptr) -> Ptr {
        let mut index = (*key) as usize % TABLE_SIZE;
        let mut tomb: *mut u8 = ptr::null_mut();

        loop {
            unsafe {
                let entry = self.entries.add(Self::size_of() * index);
                if get_key(entry).is_null() {
                    if get_value(entry).is_none() {
                        return if tomb.is_null() {
                            entry
                        } else {
                            tomb
                        }
                    } else {
                        tomb = entry.clone();
                    }

                } else if get_key(entry) == *key {
                    return entry;
                }

                index = (index + 1) % TABLE_SIZE;
            }
        }
    }

    pub unsafe fn get(&self, key: &Ptr) -> Option<Value> {
        let entry = self.find_entry(key);
        if get_key(entry).is_null() {
            return None;
        }

        Some(get_value(entry).unwrap())
    }

    pub unsafe fn set(&mut self, key: Ptr, val: Value) {
        let entry = self.find_entry(&key);

        if get_key(entry).is_null() {
            self.count += 1;
        }

        set_key(entry, key);
        set_value(entry, val);
    }

    pub unsafe fn delete(&mut self, key: Ptr) {
        if self.count == 0 {
            return;
        }

        let entry = self.find_entry(&key);

        set_key(entry, ptr::null::<u8>() as Ptr);
        set_value(entry, Value::Bool(true))
    }
}

mod tests {
    use crate::hash_table::HashTable;
    use crate::heap::Ptr;
    use crate::value::Value;

    #[test]
    pub fn test_set() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = 12usize as Ptr;
            table.set(key1.clone(), Value::Number(12f32));

            let val = table.get(&key1).unwrap();
            assert!(matches!(val, Value::Number(12f32)));
        }
    }

    #[test]
    pub fn test_set_collisions() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = 12usize as Ptr;
            table.set(key1.clone(), Value::Number(12f32));

            let key2 = 62usize as Ptr;
            table.set(key2.clone(), Value::Number(13f32));

            let val = table.get(&key1).unwrap();
            assert!(matches!(val, Value::Number(12f32)));

            let val = table.get(&key2).unwrap();
            assert!(matches!(val, Value::Number(13f32)));
        }
    }

    #[test]
    pub fn test_delete() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = 12usize as Ptr;
            table.set(key1.clone(), Value::Number(12f32));

            let key2 = 62usize as Ptr;
            table.set(key2.clone(), Value::Number(13f32));

            table.delete(key1);
            let val = table.get(&key1);
            assert!(matches!(val, None));
        }
    }

    #[test]
    pub fn test_delete_and_get() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = 12usize as Ptr;
            table.set(key1.clone(), Value::Number(12f32));

            let key2 = 62usize as Ptr;
            table.set(key2.clone(), Value::Number(13f32));

            table.delete(key1);
            let val = table.get(&key1);
            assert!(matches!(val, None));

            let val2 = table.get(&key2).unwrap();
            assert!(matches!(val2, Value::Number(13f32)));
        }
    }

    #[test]
    pub fn test_delete_get_and_set() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = 12usize as Ptr;
            table.set(key1.clone(), Value::Number(12f32));

            let key2 = 62usize as Ptr;
            table.set(key2.clone(), Value::Number(13f32));

            table.delete(key1);
            let val = table.get(&key1);
            assert!(matches!(val, None));

            let mut val2 = table.get(&key2).unwrap();
            assert!(matches!(val2, Value::Number(13f32)));

            let key3 = 112usize as Ptr;
            table.set(key3.clone(), Value::Number(14f32));

            let val3 = table.get(&key3).unwrap();
            assert!(matches!(val3, Value::Number(14f32)));

            val2 = table.get(&key2).unwrap();
            assert!(matches!(val2, Value::Number(13f32)));
        }
    }
}