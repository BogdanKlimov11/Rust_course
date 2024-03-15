fn gnome_sort<T: Ord>(arr: &mut [T]) {
    let mut i = 0;
    while i < arr.len() {
        if i == 0 || arr[i - 1] <= arr[i] {
            i += 1;
        } else {
            arr.swap(i, i - 1);
            i -= 1;
        }
    }
}
