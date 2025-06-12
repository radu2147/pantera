use crate::heap::Ptr;
use crate::utils::vec_to_array;

// > Readers

pub const NULL: u8 = 0u8;

pub unsafe fn read_byte(entry: Ptr) -> u8 {
    *entry
}

pub unsafe fn read_bytes(entry: Ptr, len: usize) -> Vec<u8> {
    let mut bytes = vec![];
    for i in 0..len {
        bytes.push(*entry.add(i));
    }

    bytes
}

pub unsafe fn read_bytes_until_null(entry: Ptr) -> Vec<u8> {
    let mut bytes = vec![];
    let mut it_ptr = entry;
    loop {
        if *it_ptr == NULL {
            break;
        }
        bytes.push(*it_ptr);
        it_ptr = it_ptr.add(1);
    }

    bytes
}

pub unsafe fn read_number(entry: Ptr) -> f64 {
    let bytes = read_bytes(entry, size_of::<Ptr>());
    let bytes_as_arr = vec_to_array(&bytes);

    f64::from_le_bytes(bytes_as_arr)
}

pub unsafe fn read_string(entry: Ptr) -> String {
    String::from_utf8(read_bytes_until_null(entry)).unwrap()
}

pub unsafe fn read_pointer(entry: Ptr) -> Ptr {
    let ptr_num = read_number(entry) as usize;

    ptr_num as Ptr
}

// < Readers

// > Writers

pub unsafe fn write_byte(dest: Ptr, byte: u8) {
    *dest = byte;
}

pub unsafe fn write_bytes(dest: Ptr, bytes: &Vec<u8>) {
    for i in 0..bytes.len() {
        write_byte(dest.add(i), *bytes.get(i).unwrap())
    }
}

pub unsafe fn write_string(dest: Ptr, string: String) {
    let len = string.len();
    write_bytes(dest, &string.into_bytes());
    write_byte(dest.add(len), NULL);
}

pub unsafe fn write_number(dest: Ptr, num: f64) {
    write_bytes(dest, &num.to_le_bytes().to_vec());
}

pub unsafe fn write_pointer(dest: Ptr, ptr: Ptr) {
    write_number(dest, (ptr as usize) as f64);
}

// < Writers

mod test {
    use std::alloc::{alloc, Layout};
    use crate::bytes::{read_bytes, read_number, read_pointer, read_string, write_byte, write_bytes, write_number, write_pointer, write_string};
    use crate::heap::Ptr;

    #[test]
    pub fn test_writer() {
        unsafe {
            let layout = Layout::array::<u8>(8).unwrap();
            let obj_ptr = alloc(layout);

            write_byte(obj_ptr, 23);
            assert_eq!(*obj_ptr, 23);

            let bytes = vec![1, 2, 3, 23, 53, 1, 2];
            write_bytes(obj_ptr, &bytes);
            for i in 0..bytes.len() {
                assert_eq!(bytes[i], *obj_ptr.add(i));
            }
        }
    }

    #[test]
    pub fn test_reader() {
        unsafe {
            let layout = Layout::array::<u8>(8).unwrap();
            let obj_ptr = alloc(layout);

            write_byte(obj_ptr, 23);
            assert_eq!(read_bytes(obj_ptr, 1)[0], 23);

            let bytes = vec![1, 2, 3, 23, 53, 1, 2];
            write_bytes(obj_ptr, &bytes);
            let read_bytes = read_bytes(obj_ptr, bytes.len());

            for i in 0..bytes.len() {
                assert_eq!(bytes[i], read_bytes[i]);
            }
        }
    }

    #[test]
    pub fn test_number() {
        unsafe {
            let layout = Layout::array::<u8>(8).unwrap();
            let obj_ptr = alloc(layout);

            write_number(obj_ptr, 123456f64);
            assert_eq!(read_number(obj_ptr), 123456f64);

            write_number(obj_ptr, -123.12f64);
            assert_eq!(read_number(obj_ptr), -123.12f64);
        }
    }

    #[test]
    pub fn test_pointer() {
        unsafe {
            let layout = Layout::array::<u8>(8).unwrap();
            let obj_ptr = alloc(layout);
            let mut x = 122u8;
            let x_ptr = &mut x as Ptr;

            write_pointer(obj_ptr, x_ptr.clone());
            let read_ptr = read_pointer(obj_ptr);

            assert_eq!(read_ptr, x_ptr);
            assert_eq!(*read_ptr, x);
        }
    }

    #[test]
    pub fn test_string() {
        unsafe {
            let layout = Layout::array::<u8>(29).unwrap();
            let obj_ptr = alloc(layout);

            write_string(obj_ptr, "test_smt".to_string());
            let read_str = read_string(obj_ptr);
            assert_eq!(read_str, "test_smt".to_string());
        }
    }
}