/*
    Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
    You may assume that each input would have exactly one solution, and you may not use the same element twice.
    You can return the answer in any order.


    Example 1:

    Input: nums = [2,7,11,15], target = 9
    Output: [0,1]
    Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

    Example 2:

    Input: nums = [3,2,4], target = 6
    Output: [1,2]

    Example 3:

    Input: nums = [3,3], target = 6
    Output: [0,1]

    

    Constraints:
        2 <= nums.length <= 104
        -109 <= nums[i] <= 109
        -109 <= target <= 109
        Only one valid answer exists.
*/
// use std::collections::HashMap;

// fn two_sum() -> [i32; 2] {
    
//     let mut values: HashMap<i32, i32> = HashMap::new();

//     let nums: [i32; 4] = [2,7,11,15];
//     let target: i32 = 9;

//     for (idx, value) in nums.iter().enumerate() {
//         let diff:i32 = target - value;
//         if values.contains_key(&diff) {
//             println!("{}", diff);
//             let first:i32 = *values.get(&diff).unwrap();
//             let ret: [i32; 2] = [first, idx.try_into().unwrap()];

//             return ret;
//         }
//         else
//         {
//             values.insert(*value, idx.try_into().unwrap());
//             println!("{}", diff);
//         }
//     }
//     let default: [i32; 2] = [0, 0];
//     return default

// }

fn merge_sort(arr: & [i32]) {

    let (l, r) = arr.split_at(3);
    print!("{}, {}", l.len(), r.len());


}

fn main() {
    let nums: [i32; 4] = [2,7,11,15];
    merge_sort(& nums);
}
