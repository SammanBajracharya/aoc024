#[allow(dead_code)]
pub enum SortType {
    Asc,  // Ascending order
    Desc, // Descending order
}

#[allow(dead_code)]
pub fn quicksort<T: Ord>(arr: &mut [T], sort_type: SortType) {
    match sort_type {
        SortType::Asc => _quicksort(arr, 0, (arr.len() - 1) as isize, true),
        SortType::Desc => _quicksort(arr, 0, (arr.len() - 1) as isize, false),
    }
}

fn _partition<T: Ord>(arr: &mut [T], left: isize, right: isize, asc: bool) -> isize {
    let pivot = right;
    let mut i: isize = left - 1;

    for j in left..=right - 1 {
        if (asc && arr[j as usize] <= arr[pivot as usize]) || (!asc && arr[j as usize] >= arr[pivot as usize]) {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);
    i + 1
}

fn _quicksort<T: Ord>(arr: &mut [T], left: isize, right: isize, asc: bool) {
    if left < right {
        let partition_idx = _partition(arr, left, right, asc);

        _quicksort(arr, left, partition_idx - 1, asc);
        _quicksort(arr, partition_idx + 1, right, asc);
    }
}

