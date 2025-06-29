use std::alloc::{alloc, Layout, LayoutError};
use std::collections::HashMap;
use crate::bytes::{read_byte, read_bytes_until_null, read_string, write_byte, write_string, NULL};
use crate::hash_table::HashTable;
use crate::types::Type;
use crate::value::Value;

pub type Ptr = *mut u8;

#[derive(Debug)]
pub struct HeapManager {
    pub interned_strings: Vec<Ptr>
}

impl HeapManager {

    pub fn new() ->Self {
        Self {
            interned_strings: vec![]
        }
    }

    // > Object
    pub fn allocate_object(&mut self, val: HashMap<Ptr, Value>) -> Result<Ptr, LayoutError> {
        unsafe {
            let mut map = HashTable::new();

            for (key, val) in val.into_iter() {
                map.set(key, val);
            }

            Ok(map.entries)
        }
    }

    pub fn get_object(obj_ptr: Ptr) -> HashMap<Ptr, Box<Value>> {
        unsafe {
            let mut map = HashTable::from(obj_ptr);
            let elems = map.get_all();

            let mut rez = HashMap::new();
            elems.into_iter().for_each(|entry| {
                let entr = map.get(&entry.key).unwrap();
                rez.insert(entry.key, Box::from(entr));
            });

            rez
        }
    }

    pub const fn get_object_entry_size() -> usize {
        size_of::<Ptr>()
    }

    pub fn get_value_from_bytes(value_bytes: Vec<u8>, typ: Type) -> Value {
        match typ {
            Type::String => {
                let mut bytes :[u8;Self::get_object_entry_size()] = [0u8;Self::get_object_entry_size()];
                for i in 0..Self::get_object_entry_size() {
                    bytes[i] = value_bytes[i];
                }
                let ptr = u64::from_le_bytes(bytes) as Ptr;
                Value::String(ptr)
            },
            Type::Object => {
                let mut bytes :[u8;Self::get_object_entry_size()] = [0u8;Self::get_object_entry_size()];
                for i in 0..Self::get_object_entry_size() {
                    bytes[i] = value_bytes[i];
                }
                let ptr = u64::from_le_bytes(bytes) as Ptr;
                Value::Object(ptr)
            },
            Type::Null => Value::Null.into(),
            Type::Number => {
                let mut arr: [u8; 8] = [0u8; 8];
                for i in 0..8 {
                    arr[i] = value_bytes[i];
                }

                Value::Number(f64::from_le_bytes(arr) as f32).into()
            },
            Type::Boolean => {
                Value::Bool(value_bytes.get(0).is_some_and(|val| *val == 1)).into()
            },
            Type::Function => {
                let mut arr: [u8; 4] = [0u8; 4];
                for i in 0..4 {
                    arr[i] = value_bytes[i];
                }
                Value::Function(u32::from_le_bytes(arr) as usize, 0).into()
            },
            _ => panic!("")
        }
    }

    pub fn get_property_from_object(&self, obj_ptr: Ptr, name: &Ptr) -> Value {
        unsafe {
            let mut map = HashTable::from(obj_ptr);
            let elem = map.get(&name);

            if elem.is_none() {
                return Value::Null;
            }

            elem.unwrap()
        }
    }

    pub fn set_property_for_object(&self, obj_ptr: Ptr, key: Ptr, val: Value) {
        unsafe {
            let mut map = HashTable::from(obj_ptr);
            map.set(key, val);
        }
    }

    pub fn compare_objects(obj1: Ptr, obj2: Ptr) -> bool {
        obj1 == obj2
    }

    pub fn concatenate_objects(&mut self, obj1: Ptr, obj2: Ptr) -> Ptr {
        unsafe {
            let mut obj_main = HashTable::from(obj1);
            let obj_sec = HashTable::from(obj2);

            obj_sec.get_all().into_iter().for_each(|en| {
                obj_main.set(en.key, en.value);
            });

            obj_main.entries
        }
    }

    // < Object

    // > Strings

    pub fn get_string(str_ptr: Ptr) -> String {
        unsafe {
            read_string(str_ptr.add(1))
        }
    }

    pub fn concatenate_strings(&mut self, string1: Ptr, string2: Ptr) -> Ptr {
        unsafe {
            let mut bytes1 = read_bytes_until_null(string1);
            let bytes2 = read_bytes_until_null(string2);

            bytes2.into_iter().for_each(|bt| bytes1.push(bt));

            self.allocate_string(String::from_utf8(bytes1).unwrap()).unwrap()
        }
    }

    pub fn compare_strings(string1: Ptr, string2: Ptr) -> bool {
        string1 == string2
    }

    fn check_string_is_interned(&self, string: &str) -> Option<Ptr> {
        for st in &self.interned_strings {
            unsafe {
                let interned_str = read_string(st.add(1));
                if interned_str == string {
                    return Some(*st);
                }
            }
        }

        None
    }

    pub fn allocate_string(&mut self, string: String) -> Result<Ptr, LayoutError> {
        if let Some(existing_str) = self.check_string_is_interned(&string) {
            return Ok(existing_str);
        }
        unsafe {
            let layout = Layout::array::<u8>(string.len() + 1 + 1)?;
            let ptr = alloc(layout);

            write_byte(ptr, Type::String as u8);
            write_string(ptr.add(1), string);

            self.interned_strings.push(ptr);

            Ok(ptr)
        }
    }

    // > Strings
}