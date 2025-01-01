pub fn quick_sort(list: &mut [i32], low: usize, high: usize) -> () {
    if list.len() <= 1 || low >= high {
        return;
    }

    let mut pivot_position: usize = high;

    let mut swap_position: usize = low;
    let mut swap_activated: bool = false; // necessary because usize cannot be -1

    for index in low..pivot_position {
        if list[index] < list[pivot_position] {
            if swap_activated {
                swap_position += 1;
            } else {
                swap_activated = true;
            }
            list.swap(index, swap_position);
        }
    }

    if swap_activated {
        list.swap(swap_position + 1, pivot_position);
        pivot_position = swap_position + 1;
    } else {
        list.swap(swap_position, pivot_position);
        pivot_position = swap_position;
    }

    if pivot_position != 0 {
        quick_sort(list, low, pivot_position - 1);
    }
    quick_sort(list, pivot_position + 1, high);
}
