/// 11. Container with Most Water
///
/// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
///
/// Find two lines that together with the x-axis form a container, such that the container contains the most water.
///
/// Return the maximum amount of water a container can store.
///
/// Notice that you may not slant the container.
///
/// Example:
///                  l               r
/// Input: height = [0,1,2,3,4,5,6,7,8]
/// Input: height = [1,8,6,2,5,4,8,3,7]
/// Output: 49

pub fn main(height: Vec<i32>) -> i32 {
    let mut area = 0;

    let (mut l, mut r) = (0, height.len() - 1);

    while l < r {
        area = area.max((height[l].min(height[r])) * (r - l) as i32);

        if height[l] > height[r] {
            r -= 1;
        } else {
            l += 1;
        }
    }

    area
}
