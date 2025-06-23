use std::alloc::{alloc, Layout, LayoutError};
use std::collections::HashMap;
use crate::bytes::{read_byte, read_bytes_until_null, read_string, write_byte, write_string, NULL};
use crate::hash_table::HashTable;
use crate::types::Type;
use crate::value::Value;

pub type Ptr = *mut u8;

#[derive(Debug)]
pub struct HeapManager {}

impl HeapManager {

    pub fn new() ->Self {
        Self {}
    }

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
            let mut map = HashTable::from(obj_ptr.sub(1));
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

    pub fn compute_object_byte_size(val: &HashMap<Ptr, Value>) -> usize {
        let keys_length = val.keys().len() * Self::get_object_entry_size();
        let values_length = val.values().len() * Self::get_object_entry_size();

        keys_length + values_length + 1usize + 4usize
    }

    pub fn get_raw_value(value_bytes: Vec<u8>, typ: Type) -> Value {
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
        // let binding = HashMap::new();
        // let HeapValue::String(obj_prop) = Self::get_from_heap(*name) else {panic!("Unreachable");};
        // let offset = self.object_keys.get(&obj_ptr).unwrap_or(&binding).get(&obj_prop);
        // if let Some(off) = offset {
        //     unsafe {
        //         let mut val_ptr = obj_ptr.add(1 + 4 + Self::get_object_entry_size() * (*off + 1) + (Self::get_object_entry_size() +1) * (*off));
        //         let mut val_as_bytes: Vec<u8> = vec![];
        //         let typ = Type::from(*val_ptr);
        //         val_ptr = val_ptr.add(1);
        //
        //         for _j in 0..Self::get_object_entry_size() {
        //             val_as_bytes.push(*val_ptr);
        //             val_ptr = val_ptr.add(1);
        //         }
        //
        //         Self::get_raw_value(val_as_bytes, typ)
        //     }
        // } else {
        //     Value::Null
        // }
        Value::Null
    }

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
        unsafe {
            let mut it_str1 = string1;
            let mut it_str2 = string2;
            loop {
                if read_byte(it_str1) != read_byte(it_str2) {
                    return false;
                }
                if read_byte(it_str1) == NULL {
                    break;
                }

                it_str1 = it_str1.add(1);
                it_str2 = it_str2.add(1);

            }

            true
        }
    }

    pub fn allocate_string(&mut self, string: String) -> Result<Ptr, LayoutError> {
        unsafe {
            let layout = Layout::array::<u8>(string.len() + 1 + 1)?;
            let ptr = alloc(layout);

            write_byte(ptr, Type::String as u8);
            write_string(ptr.add(1), string);

            Ok(ptr)
        }
    }

    // > Strings
}