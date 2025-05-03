use std::alloc::{alloc, Layout, LayoutError};
use crate::types::Type;

pub type Ptr = *mut u8;

pub enum HeapValue {
    String(String),
    Object
}

#[derive(Debug)]
pub struct HeapManager {}

impl HeapManager {

    pub fn new() ->Self {
        Self {}
    }
    pub fn allocate_object(&mut self, typ: Type, object_as_bytes: &[u8]) -> Result<Ptr, LayoutError> {
        match typ {
            Type::String => self.allocate_string(object_as_bytes),
            _ => panic!("Cannot allocate this type")
        }
    }

    pub fn concatenate_strings(&mut self, string1: Ptr, string2: Ptr) -> Ptr {
        unsafe {
            let mut len1_as_bytes = [0u8; 4];
            let mut len2_as_bytes = [0u8; 4];
            for i in 1..5 {
                len1_as_bytes[i - 1] = *string1.wrapping_add(i);
                len2_as_bytes[i - 1] = *string2.wrapping_add(i);
            }

            let mut bytes = vec![];

            for i in 0..u32::from_le_bytes(len1_as_bytes) as usize {
                bytes.push(*string1.wrapping_add(i + 5usize));
            }

            for i in 0..u32::from_le_bytes(len2_as_bytes) as usize {
                bytes.push(*string2.wrapping_add(i + 5usize));
            }

            self.allocate_string(bytes.as_slice()).unwrap()
        }
    }

    pub fn compare_strings(string1: Ptr, string2: Ptr) -> bool {
        unsafe {
            if *string1 != *string2 {
                return false;
            }
            let mut len_as_bytes = [0u8; 4];
            for i in 1..5 {
                len_as_bytes[i - 1] = *string1.wrapping_add(i);
                if *string1.wrapping_add(i) != *string2.wrapping_add(i) {
                    return false;
                }
            }
            let len = u32::from_le_bytes(len_as_bytes);
            for i in 0..len as usize {
                if *string1.wrapping_add(i + 4usize +1usize) != *string2.wrapping_add(i + 4usize +1usize) {
                    return false;
                }
            }

            true
        }
    }

    pub fn get_object(ptr: Ptr) -> HeapValue {
        unsafe {
            let byte = *ptr;
            match Type::from(byte) {
                Type::String => HeapValue::String(Self::get_string(ptr.wrapping_add(1))),
                _ => panic!("Object is not allocated on heap")
            }
        }
    }

    fn get_string(ptr: Ptr) -> String {
        unsafe {
            let mut len_as_bytes = [0u8; 4];
            for i in 0..4 {
                len_as_bytes[i] = *ptr.wrapping_add(i);
            }
            let len = u32::from_le_bytes(len_as_bytes);
            let mut string_as_bytes: Vec<u8> = vec![];
            for i in 0..len as usize {
                string_as_bytes.push(*ptr.wrapping_add(i + 4usize));
            }
            String::from_utf8(string_as_bytes).unwrap()
        }
    }

    fn allocate_string(&mut self, string_as_bytes: &[u8]) -> Result<Ptr, LayoutError> {
        unsafe {
            let layout = Layout::array::<u8>(string_as_bytes.len() + 1 + 4)?;
            let ptr = alloc(layout);

            *ptr = Type::String.into();

            for i in 0..string_as_bytes.len().to_le_bytes().len() {
                *ptr.wrapping_add(i + 1) = string_as_bytes.len().to_le_bytes()[i];
            }


            for i in 0..string_as_bytes.len() {
                *ptr.wrapping_add(i + 1 + 4) = string_as_bytes[i];
            }

            Ok(ptr)
        }
    }
}