pub fn vec_to_array<T: Clone + Default + Copy>(vector: &Vec<T>) -> [T;8] {
    let mut array = [T::default();8];
    for i in 0..8 {
        let el = *vector.get(i).unwrap();
        array[i] = el;
    }

    array
}

mod tests {
    

    #[test]
    fn test_vec_to_array() {
        let vr = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let arr = vec_to_array(&vr);
        for i in 0..8 {
            assert_eq!(*vr.get(i).unwrap(), arr[i])
        }
    }
}