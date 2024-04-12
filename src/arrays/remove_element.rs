/// Leetcode 150
///
/// 27. Remove Element
///
/// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
///
/// Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
///
/// return `k`
///
/// Example
/// > Input: nums = [3,2,2,3], val = 3
/// > Output: 2, nums = [2,2,_,_]
/// > Explanation: Your function should return k = 2, with the first two elements of nums being 2.
/// > It does not matter what you leave beyond the returned k (hence they are underscores).

pub fn main(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    for num in 0..nums.len() {
        if nums[num] != val {
            k += 1;
        } else {
            nums[num] = -1;
        }
    }

    nums.sort_by(|a, b| b.cmp(a));

    k
}
