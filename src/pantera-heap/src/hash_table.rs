use std::alloc::{alloc, Layout};
use std::ptr;
use crate::bytes::{read_byte, read_bytes, read_number, read_pointer, write_bool, write_byte, write_number, write_pointer};
use crate::heap::{HeapManager, Ptr};
use crate::types::Type;
use crate::value::Value;

const TABLE_SIZE: usize = 50;

#[derive(Debug)]
pub struct HashEntry {
    pub key: Ptr,
    pub value: Value
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

#[allow(dead_code)]
pub unsafe fn set_type(entry: Ptr, typ: Type) {
    write_byte(entry.add(8), typ.into());
}

pub unsafe fn get_value(entry: Ptr) -> Option<Value> {
    let typ = get_type(entry);
    if matches!(typ, Type::Empty) {
        return None;
    }

    let bytes = read_bytes(entry.add(8 + 1), 8);

    let val = HeapManager::get_value_from_bytes(bytes, get_type(entry));
    Some(val)
}

pub unsafe fn set_value(entry: Ptr, value: Value) {
    let dest = entry.add(8);
    match value {
        Value::Number(num) => {
            write_byte(dest, Type::Number as u8);
            write_number(dest.add(1), num as f64);
        }
        Value::Bool(bl) => {
            write_byte(dest, Type::Boolean as u8);
            write_bool(dest.add(1), bl);
        }
        Value::Null => {
            write_byte(dest, Type::Null as u8);
            write_number(dest.add(1), 0f64);
        }
        Value::Function(func_ptr, _) => {
            write_byte(dest, Type::Function as u8);
            write_pointer(dest.add(1), func_ptr as Ptr);
        },
        Value::String(str_ptr) => {
            *dest = Type::String as u8;
            write_pointer(dest.add(1), str_ptr);
        }
        Value::Object(obj_ptr) => {
            *dest = Type::Object as u8;
            write_pointer(dest.add(1), obj_ptr);
        },
        Value::Array(arr_ptr) => {
            *dest = Type::Array as u8;
            write_pointer(dest.add(1), arr_ptr);
        }
    }
}


pub struct HashTable {
    pub entries: *mut u8,
    count: usize,
    pub layout: Option<Layout>
}

impl HashTable {
    pub unsafe fn new() -> Self {
        let layout = Layout::array::<u8>(TABLE_SIZE * Self::size_of() + 1 + 8).unwrap();
        let obj_ptr = alloc(layout);
        write_byte(obj_ptr, Type::Object.into());

        Self {
            entries: obj_ptr,
            count: 0,
            layout: Some(layout)
        }
    }

    pub unsafe fn from(obj_ptr: Ptr) -> Self {
        Self {
            entries: obj_ptr,
            count: read_number(obj_ptr.add(1)) as usize,
            layout: None
        }
    }

    fn size_of() -> usize {
        8 + (1 + 8)
    }

    unsafe fn get_values_start(&self) -> Ptr {
        self.entries.add(1 + 8)
    }

    fn find_entry(&self, key: &Ptr) -> Ptr {
        let mut index = (*key) as usize % TABLE_SIZE;
        let mut tomb: *mut u8 = ptr::null_mut();

        loop {
            unsafe {
                let entry = self.get_values_start().add(Self::size_of() * index);
                if get_key(entry).is_null() {
                    if get_value(entry).is_none() {
                        return if tomb.is_null() {
                            entry
                        } else {
                            tomb
                        }
                    } else {
                        tomb = entry;
                    }

                } else if get_key(entry) == *key {
                    return entry;
                }

                index = (index + 1) % TABLE_SIZE;
            }
        }
    }

    pub unsafe fn get_all(&self) -> Vec<HashEntry> {
        let mut entries = vec![];

        let mut it_ptr = self.get_values_start();
        for _i in 0..TABLE_SIZE {
            let key = get_key(it_ptr);
            if !key.is_null() {
                let value = get_value(it_ptr).unwrap();

                entries.push(HashEntry { key, value });
            }

            it_ptr = it_ptr.add(Self::size_of());
        }

        entries
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
            self.set_count(self.count + 1);
        }

        set_key(entry, key);
        set_value(entry, val);
    }

    #[allow(dead_code)]
    pub unsafe fn delete(&mut self, key: Ptr) {
        if self.count == 0 {
            return;
        }

        let entry = self.find_entry(&key);

        set_key(entry, ptr::null::<u8>() as Ptr);
        set_value(entry, Value::Bool(true));
        self.set_count(self.count - 1);
    }

    unsafe fn set_count(&mut self, count: usize) {
        self.count = count;
        write_number(self.entries.add(1), count as f64);
    }

    #[allow(dead_code)]
    pub fn get_count(&self) -> usize {
        self.count
    }
}

mod tests {
    use std::alloc::{alloc, Layout};
    
    use crate::bytes::{write_byte, write_string};
    
    use crate::heap::Ptr;
    use crate::types::Type;


    #[allow(dead_code)]
    unsafe fn alloc_key(str: String) -> Ptr {
        let layout = Layout::array::<u8>(str.len()).unwrap();
        let ptr = alloc(layout);

        write_byte(ptr, Type::String as u8);
        write_string(ptr.add(1), str);

        ptr
    }

    #[test]
    pub fn test_set() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = Rc::new(alloc_key("Test".to_string()));
            table.set(*key1, Value::Number(12f32));

            let val = table.get(&key1).unwrap();
            assert!(matches!(val, Value::Number(12f32)));
        }
    }

    #[test]
    pub fn test_set_collisions() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = Rc::new(alloc_key("Test1".to_string()));
            table.set(*key1, Value::Number(12f32));

            let key2 = Rc::new(alloc_key("Test2".to_string()));
            table.set(*key2, Value::Number(13f32));

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
            let key1 = Rc::new(12usize as Ptr);
            table.set(*key1, Value::Number(12f32));

            let key2 = Rc::new(62usize as Ptr);
            table.set(*key2, Value::Number(13f32));

            table.delete(*key1);
            let val = table.get(&key1);
            assert!(matches!(val, None));
        }
    }

    #[test]
    pub fn test_delete_and_get() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = Rc::new(12usize as Ptr);
            table.set(*key1, Value::Number(12f32));

            let key2 = Rc::new(62usize as Ptr);
            table.set(*key2, Value::Number(13f32));

            table.delete(*key1);
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
            let key1 = Rc::new(12usize as Ptr);
            table.set(*key1, Value::Number(12f32));

            let key2 = Rc::new(62usize as Ptr);
            table.set(*key2, Value::Number(13f32));

            table.delete(*key1);
            let val = table.get(&key1);
            assert!(matches!(val, None));

            let mut val2 = table.get(&key2).unwrap();
            assert!(matches!(val2, Value::Number(13f32)));

            let key3 = Rc::new(112usize as Ptr);
            table.set(*key3, Value::Number(14f32));

            let val3 = table.get(&key3).unwrap();
            assert!(matches!(val3, Value::Number(14f32)));

            val2 = table.get(&key2).unwrap();
            assert!(matches!(val2, Value::Number(13f32)));
        }
    }

    #[test]
    pub fn test_from() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = 12usize as Ptr;
            table.set(key1, Value::Number(12f32));

            let key2 = 62usize as Ptr;
            table.set(key2, Value::Number(13f32));

            assert_eq!(table.get_count(), 2);

            let tbl_cpy = HashTable::from(table.entries);
            assert_eq!(tbl_cpy.get_count(), 2);
        }
    }

    #[test]
    pub fn test_get_all() {
        unsafe {
            let mut table = HashTable::new();
            let key1 = 12usize as Ptr;
            table.set(key1, Value::Number(12f32));

            let all = table.get_all();
            assert_eq!(all.len(), 1);
        }
    }
}