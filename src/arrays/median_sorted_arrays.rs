use std::ops::Sub;

/// 4. Median of Two Sorted Arrays
///
/// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
///
/// The overall run time complexity should be `O(log (m+n))`.
///
/// Example 1:
/// > Input: nums1 = [1,3], nums2 = [2]
/// > Output: 2.00000
/// > Explanation: merged array = [1,2,3] and median is 2.
///
/// Example 2:
/// > Input: nums1 = [1,2], nums2 = [3,4]
/// > Output: 2.50000
/// > Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.

pub fn main(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged_array = Vec::<i32>::new();

    merged_array.append(&mut nums1.clone());
    merged_array.append(&mut nums2.clone());

    merged_array.sort();

    if merged_array.len() % 2 != 0 {
        // odd length
        let index = merged_array.len().div_ceil(2).sub(1);
        return f64::from(merged_array.get(index).cloned().unwrap_or(0));
    } else {
        // even length
        let index = merged_array.len().div_ceil(2).sub(1);
        let current = merged_array.get(index).cloned().unwrap() as f64;
        let fwd = merged_array.get(index + 1).cloned().unwrap() as f64;

        return (current + fwd) / 2.00;
    }
}
