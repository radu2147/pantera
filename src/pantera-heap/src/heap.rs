use std::alloc::{alloc, Layout, LayoutError};
use std::collections::HashMap;
use crate::bytes::{read_byte, read_bytes, read_bytes_until_null, read_string, write_byte, write_string, NULL};
use crate::types::Type;
use crate::value::{HeapValue, Value};

pub type Ptr = *mut u8;

#[derive(Debug)]
pub struct HeapManager {}

impl HeapManager {

    pub fn new() ->Self {
        Self {}
    }

    pub fn allocate_value(val: &Value, dest: Ptr) {
        unsafe {
            match val {
                Value::Number(num) => {
                    *dest = Type::Number as u8;
                    let num_as_bytes = num.to_le_bytes();
                    let padding = Self::get_object_entry_size() - num_as_bytes.len();
                    for i in 0..Self::get_object_entry_size() {
                        if i < padding {
                            *dest.wrapping_add(i + 1) = 0u8;
                        } else {
                            *dest.wrapping_add(i + 1) = num_as_bytes[i - padding];
                        }
                    }
                }
                Value::Bool(bl) => {
                    *dest = Type::Boolean as u8;
                    for i in 0..Self::get_object_entry_size() - 1 {
                        *dest.wrapping_add(i + 1) = 0u8;
                    }
                    *dest.wrapping_add(Self::get_object_entry_size()) = if *bl == true { 1u8 } else { 0u8 };
                }
                Value::Null => {
                    *dest = Type::Null as u8;
                    for i in 0..Self::get_object_entry_size() {
                        *dest.wrapping_add(i + 1) = 0u8;
                    }
                }
                Value::Function(func_ptr, _) => {
                    *dest = Type::Function as u8;
                    let num_as_bytes = (*func_ptr as f32).to_le_bytes();
                    let padding = Self::get_object_entry_size() - num_as_bytes.len();
                    for i in 0..Self::get_object_entry_size() {
                        if i < padding {
                            *dest.wrapping_add(i + 1) = 0u8;
                        } else {
                            *dest.wrapping_add(i + 1) = num_as_bytes[i - padding];
                        }
                    }
                },
                Value::String(str_ptr) => {
                    *dest = Type::String as u8;
                    Self::allocate_pointer(dest.add(1), *str_ptr);
                }
                Value::Object(obj_ptr) => {
                    *dest = Type::Object as u8;
                    Self::allocate_pointer(dest.add(1), *obj_ptr);
                }
            }
        }
    }

    pub fn allocate_pointer(dest_ptr: Ptr, src_ptr: Ptr) {
        let mut it_ptr = dest_ptr;
        unsafe {
            let bytes = (src_ptr as u64).to_le_bytes();
            bytes.iter().for_each(|bt| {
                *it_ptr = *bt;
                it_ptr = it_ptr.add(1);
            }
            );
        }
    }

    pub fn allocate_object(&mut self, val: HashMap<Ptr, Value>) -> Result<Ptr, LayoutError> {
        unsafe {
            let layout = Layout::array::<u8>(Self::compute_object_byte_size(&val))?;
            let obj_ptr = alloc(layout);

            *obj_ptr = Type::Object as u8;
            let mut it_ptr = obj_ptr.add(1);

            for i in 0..val.len().to_le_bytes().len() {
                *it_ptr.add(i) = val.len().to_le_bytes()[i];
            }

            it_ptr = it_ptr.add(4);

            for (key, val) in val.iter() {
                Self::allocate_pointer(it_ptr, *key);
                it_ptr = it_ptr.add(Self::get_object_entry_size());

                HeapManager::allocate_value(val, it_ptr);
                it_ptr = it_ptr.add(Self::get_object_entry_size() + 1);
            }

            Ok(obj_ptr)
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

    pub fn get_from_heap(ptr: Ptr) -> HeapValue {
        unsafe {
            let byte = *ptr;
            match Type::from(byte) {
                Type::String => HeapValue::String(read_string(ptr.add(1))),
                Type::Object => HeapValue::Object(Self::get_object(ptr.add(1))),
                _ => panic!("Object is not allocated on heap")
            }
        }
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
            _ => {
                let HeapValue::Value(raw_val) = Self::get_value(value_bytes, typ) else { panic!("Something went wrong"); };
                raw_val
            }
        }
    }

    pub fn get_value(value_bytes: Vec<u8>, typ: Type) -> HeapValue {
        assert_eq!(value_bytes.len(), 8);
        match typ {
            Type::Null => Value::Null.into(),
            Type::Number => {
                let padding = Self::get_object_entry_size() - size_of::<f32>();
                let mut arr: [u8; 4] = [0u8; 4];
                for i in 0..4 {
                    arr[i] = value_bytes[padding + i];
                }

                Value::Number(f32::from_le_bytes(arr)).into()
            },
            Type::Boolean => Value::Bool(value_bytes.get(Self::get_object_entry_size() - 1).is_some_and(|val| *val == 1)).into(),
            Type::Function => {
                let mut arr: [u8; 4] = [0u8; 4];
                for i in 0..4 {
                    arr[i] = value_bytes[i];
                }
                Value::Function(u32::from_le_bytes(arr) as usize, 0).into()
            },
            Type::String => {
                let mut bytes :[u8;Self::get_object_entry_size()] = [0u8;Self::get_object_entry_size()];
                for i in 0..Self::get_object_entry_size() {
                    bytes[i] = value_bytes[i];
                }
                let ptr = u64::from_le_bytes(bytes) as Ptr;
                Self::get_from_heap(ptr)
            },
            Type::Object => {
                let mut bytes :[u8;Self::get_object_entry_size()] = [0u8;Self::get_object_entry_size()];
                for i in 0..Self::get_object_entry_size() {
                    bytes[i] = value_bytes[i];
                }
                let ptr = u64::from_le_bytes(bytes) as Ptr;
                Self::get_from_heap(ptr)
            },
            _ => panic!("Not a type")
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

    pub fn get_object(obj_ptr: Ptr) -> HashMap<String, Box<HeapValue>> {
        unsafe {
            let mut len_as_bytes = [0u8; 4];
            for i in 0..4 {
                len_as_bytes[i] = *obj_ptr.add(i);
            }
            let mut it_ptr = obj_ptr.add(4);

            let len = u32::from_le_bytes(len_as_bytes);
            let mut map = HashMap::new();
            for _i in 0..len {
                let ptr = Self::get_pointer(it_ptr);
                let HeapValue::String(key) = Self::get_from_heap(ptr) else { panic!("Only strings should be used as keys"); };
                it_ptr = it_ptr.add(Self::get_object_entry_size());

                let mut val_as_bytes: Vec<u8> = vec![];
                let typ = Type::from(*it_ptr);
                it_ptr = it_ptr.add(1);

                for _j in 0..Self::get_object_entry_size() {
                    val_as_bytes.push(*it_ptr);
                    it_ptr = it_ptr.add(1);
                }

                let value = Box::from(Self::get_value(val_as_bytes, typ));

                map.insert(key, value);
            }

            map
        }
    }

    fn get_pointer(ptr_ptr: Ptr) -> Ptr {
        let mut pointer_bytes: [u8;Self::get_object_entry_size()] = [0u8; Self::get_object_entry_size()];
        for i in 0..Self::get_object_entry_size() {
            unsafe{
                pointer_bytes[i] = *ptr_ptr.add(i);
            }
        }

        u64::from_le_bytes(pointer_bytes) as Ptr
    }

    // > Strings
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