fn main() {
    let arr = vec![2,7,11,15];
    let target = 9;


    println!("Giá trị lớn nhất của mảng 1: {}", target);

}


fn two_sum<T: PartialOrd + Copy + std::fmt::Debug>(arr: &[T]) -> T {
    println!("{:?}", arr);
    let mut max = arr[0];
    for &i in arr {
        if i > max {
            max = i;
        }
    }
    max
}