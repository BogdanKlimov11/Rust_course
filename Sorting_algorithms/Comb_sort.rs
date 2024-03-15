fn comb_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut gap = n;
    let shrink = 1.3;

    let mut sorted = false;

    while !sorted {
        gap = (gap as f64 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        let mut i = 0;
        while i + gap < n {
            if arr[i] > arr[i + gap] {
                arr.swap(i, i + gap);
                sorted = false;
            }
            i += 1;
        }
    }
}
