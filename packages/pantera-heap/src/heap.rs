use std::alloc::{alloc, dealloc, Layout, LayoutError};
use std::collections::HashMap;
use crate::array::{Array, ARRAY_BYTES_SIZE};
use crate::bytes::{read_bytes_until_null, read_string, write_byte, write_string};
use crate::hash_table::{HashTable, BYTES_SIZE};
use crate::types::Type;
use crate::value::{FunctionValue, Value};

pub type Ptr = *mut u8;

#[derive(Debug)]
pub struct HeapManager {
    pub interned_strings: HashMap<Ptr, bool>,
    pub objects: HashMap<Ptr, bool>,
    heap_layout: HashMap<Ptr, Layout>,
    pub allocated_memory: usize,
    pub max_heap_size: usize,
}

impl Default for HeapManager {
    fn default() -> Self {
        Self::new(8)
    }
}

impl HeapManager {

    pub fn new(max_heap_size: usize) ->Self {
        Self {
            interned_strings: HashMap::new(),
            objects: HashMap::new(),
            heap_layout: HashMap::new(),
            allocated_memory: 0,
            max_heap_size
        }
    }

    pub fn check_oom(&self) {
        if self.allocated_memory >= self.max_heap_size {
            panic!("OOM: Max heap size has been reached");
        }
    }

    pub fn free(&mut self, ptr: Ptr) {
        let layout = self.heap_layout.remove(&ptr).unwrap();
        unsafe {
            dealloc(ptr, layout);
        }
    }

    // > Object
    pub fn allocate_object(&mut self, val: HashMap<Ptr, Value>) -> Result<Ptr, LayoutError> {
        unsafe {
            let mut map = HashTable::new();

            for (key, val) in val.into_iter() {
                map.set(key, val);
            }

            self.objects.insert(map.entries, false);
            self.heap_layout.insert(map.entries, map.layout.unwrap());
            self.allocated_memory = self.allocated_memory + BYTES_SIZE;

            self.check_oom();

            Ok(map.entries)
        }
    }

    pub fn free_object(&mut self, ptr: Ptr) {
        self.objects.remove(&ptr);
        self.free(ptr);
    }

    pub fn get_object(obj_ptr: Ptr) -> HashMap<Ptr, Box<Value>> {
        unsafe {
            let map = HashTable::from(obj_ptr);
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
            Type::Null => Value::Null,
            Type::Number => {
                let mut arr: [u8; 8] = [0u8; 8];
                for i in 0..8 {
                    arr[i] = value_bytes[i];
                }

                Value::Number(f64::from_le_bytes(arr) as f32)
            },
            Type::Boolean => {
                Value::Bool(value_bytes.first().is_some_and(|val| *val == 1))
            },
            Type::Function => {
                let arity = u8::from_le(*value_bytes.first().unwrap_or_else(|| {panic!("Wrong architecture");}));
                let mut arr: [u8; 4] = [0u8; 4];
                for i in 1..5 {
                    arr[i - 1] = value_bytes[i];
                }
                Value::Function(FunctionValue::UserDefined(u32::from_le_bytes(arr) as usize, arity))
            },
            _ => panic!("")
        }
    }

    pub fn get_property_from_object(&self, obj_ptr: Ptr, name: &Ptr) -> Value {
        unsafe {
            let map = HashTable::from(obj_ptr);
            let elem = map.get(name);

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

    // > Arrays

    pub fn allocate_array(&mut self, val: Vec<Value>) -> Result<Ptr, LayoutError> {
        unsafe {
            let len = val.len();
            let mut arr = Array::of(len);

            for (index, val) in val.into_iter().enumerate() {
                arr.set(len - 1 - index, val);
            }

            self.objects.insert(arr.entries, false);
            self.heap_layout.insert(arr.entries, arr.layout.unwrap());
            self.allocated_memory = self.allocated_memory + ARRAY_BYTES_SIZE;

            self.check_oom();

            Ok(arr.entries)
        }
    }

    pub fn get_array(obj_ptr: Ptr) -> Vec<Value> {
        unsafe {
            let arr = Array::from(obj_ptr);

            arr.get_all()
        }
    }

    pub fn get_property_from_array(&self, arr_ptr: Ptr, key: Ptr) -> Value {
        unsafe {
            let arr = Array::from(arr_ptr);
            let ind = HeapManager::get_string(key).parse::<usize>().unwrap();
            let elem = arr.get(ind);

            if elem.is_none() {
                return Value::Null;
            }

            elem.unwrap()
        }
    }

    pub fn get_property_from_array_num(&self, arr_ptr: Ptr, key: usize) -> Value {
        unsafe {
            let arr = Array::from(arr_ptr);
            let elem = arr.get(key);

            if elem.is_none() {
                return Value::Null;
            }

            elem.unwrap()
        }
    }

    pub fn set_property_for_array(&self, arr_ptr: Ptr, ind_ptr: Ptr, val: Value) {
        unsafe {
            let ind = HeapManager::get_string(ind_ptr).parse::<usize>().unwrap();
            let mut arr = Array::from(arr_ptr);
            arr.set(ind, val);
        }
    }

    pub fn set_property_for_array_num(&self, arr_ptr: Ptr, ind: usize, val: Value) {
        unsafe {
            let mut arr = Array::from(arr_ptr);
            arr.set(ind, val);
        }
    }

    pub fn free_array(&mut self, ptr: Ptr) {
        self.free_object(ptr);
    }

    // < Arrays

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
        for st in self.interned_strings.keys() {
            unsafe {
                let interned_str = read_string(st.add(1));
                if interned_str == string {
                    return Some(*st);
                }
            }
        }

        None
    }

    pub fn allocate_compiled_string(&mut self, string: String) -> Result<Ptr, LayoutError> {
        self.allocate_string_internal(string, true)
    }

    pub fn allocate_string(&mut self, string: String) -> Result<Ptr, LayoutError> {
        self.allocate_string_internal(string, false)
    }

    fn allocate_string_internal(&mut self, string: String, is_from_compilation: bool) -> Result<Ptr, LayoutError> {
        if let Some(existing_str) = self.check_string_is_interned(&string) {
            return Ok(existing_str);
        }
        let internal_string_len = string.len() + 1 + 1;
        unsafe {
            let layout = Layout::array::<u8>(internal_string_len)?;
            let ptr = alloc(layout);

            write_byte(ptr, Type::String as u8);
            write_string(ptr.add(1), string);

            self.interned_strings.insert(ptr, is_from_compilation);
            self.heap_layout.insert(ptr, layout);
            self.allocated_memory = self.allocated_memory + internal_string_len;

            self.check_oom();

            Ok(ptr)
        }
    }

    pub fn free_string(&mut self, ptr: Ptr) {
        self.interned_strings.remove(&ptr);
        self.free(ptr);
    }

    // > Strings
}