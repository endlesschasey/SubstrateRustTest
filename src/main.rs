fn bubble_base_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = [1, 2, 3, 6, 10, 4, 5, 11, 3, 10];
    println!("Unsorted array of numbers: {:?}", numbers);
    bubble_base_sort(&mut numbers);
    println!("Sorted array of numbers: {:?}", numbers);

    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array of numbers: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("Sorted array of numbers: {:?}", numbers);

    let mut chars = ['c', 'a', 'b', 'e', 'd'];
    println!("Unsorted array of characters: {:?}", chars);
    bubble_sort(&mut chars);
    println!("Sorted array of characters: {:?}", chars);
}