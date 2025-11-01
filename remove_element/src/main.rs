fn main() {
    let mut nums: Vec<i32> = vec![1,2,3,4,8,3,2,5,3,3,2];
    println!("nums: {:?}", nums);
    println!("new_nums_len: {:?}", remove_element(&mut nums, 3));
    println!("new_nums: {:?}", nums);

}


fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| *x != val);

    return nums.len().try_into().unwrap();
}
