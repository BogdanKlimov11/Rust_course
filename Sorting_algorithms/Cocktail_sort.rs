fn cocktail_sort<T: Ord>(arr: &mut [T]) {
    let mut sorted = true;
    let mut start = 0;
    let mut end = arr.len() - 1;

    while sorted {
        sorted = false;

        for i in start..end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = true;
            }
        }
        if !sorted {
            break;
        }
        sorted = false;
        end -= 1;

        for i in (start..end).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = true;
            }
        }
        start += 1;
    }
}
