fn shell_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    let mut gap = len / 2;

    while gap > 0 {
        for i in gap..len {
            let mut j = i;
            while j >= gap && arr[j - gap] > arr[j] {
                arr.swap(j - gap, j);
                j -= gap;
            }
        }
        gap /= 2;
    }
}
