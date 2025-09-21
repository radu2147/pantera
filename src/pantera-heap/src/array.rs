use std::alloc::{alloc, Layout};
use crate::bytes::{read_byte, read_bytes, read_number, write_bool, write_byte, write_number, write_pointer};
use crate::heap::{HeapManager, Ptr};
use crate::types::Type;
use crate::value::{FunctionValue, Value};

const ARRAY_SIZE: usize = 50;

pub struct Array {
    pub entries: *mut u8,
    count: usize,
    pub layout: Option<Layout>
}

pub unsafe fn get_type(entry: Ptr) -> Type {
    Type::from(read_byte(entry))
}

pub unsafe fn get_value(entry: Ptr) -> Option<Value> {
    let typ = get_type(entry);
    if matches!(typ, Type::Empty) {
        return None;
    }

    let bytes = read_bytes(entry.add(1), 8);

    let val = HeapManager::get_value_from_bytes(bytes, typ);
    Some(val)
}

pub unsafe fn set_value(dest: Ptr, value: Value) {
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
        Value::Function(func_ptr) => {
            match func_ptr {
                FunctionValue::UserDefined(func_ptr, _) => {
                    write_byte(dest, Type::Function as u8);
                    write_pointer(dest.add(1), func_ptr as Ptr);
                },
                _ => {
                    todo!();
                }
            }
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

impl Array {
    pub unsafe fn new() -> Self {
        let layout = Layout::array::<u8>(ARRAY_SIZE * Self::size_of() + 1 + 8).unwrap();
        let arr_ptr = alloc(layout);
        write_byte(arr_ptr, Type::Array.into());

        Self {
            entries: arr_ptr,
            count: 0,
            layout: Some(layout)
        }
    }

    pub unsafe fn of(len: usize) -> Self {
        let mut arr = Self::new();
        arr.set_count(len);

        arr
    }

    unsafe fn get_slot(&self, key: usize) -> Ptr {
        self.entries.add(1 + 8).add(key * Self::size_of())
    }

    pub fn size_of() -> usize {
        1 + 8
    }

    pub unsafe fn from(obj_ptr: Ptr) -> Self {
        Self {
            entries: obj_ptr,
            count: read_number(obj_ptr.add(1)) as usize,
            layout: None
        }
    }

    pub unsafe fn get_all(&self) -> Vec<Value> {
        let mut entries = vec![];

        for ind in 0..self.count {
            entries.push(self.get(ind).unwrap());
        }

        entries
    }

    pub unsafe fn get(&self, key: usize) -> Option<Value> {
        if key >= self.count {
            return None;
        }

        Some(get_value(self.get_slot(key)).unwrap())
    }

    pub unsafe fn set(&mut self, key: usize, val: Value) {
        if key > self.count {
            panic!("Index {key} out of range");
        }

        set_value(self.get_slot(key), val);
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