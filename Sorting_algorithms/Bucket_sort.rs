fn bucket_sort(arr: &mut [f64]) {
    let mut buckets: Vec<Vec<f64>> = vec![vec![]; arr.len()];

    for &num in arr.iter() {
        let index = (num * arr.len() as f64) as usize;
        buckets[index].push(num);
    }

    for bucket in buckets.iter_mut() {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let mut index = 0;
    for bucket in buckets.iter() {
        for &num in bucket.iter() {
            arr[index] = num;
            index += 1;
        }
    }
}
