/* Problem Statement
Given an array of integers nums and a target integer, return the indices of the two numbers that add up to the target. Constraints include:

 - Exactly one valid solution exists.

 - The same element cannot be used twice */

 //Time Complexity O(n^2), Space Complexity O(n), Slow for larger arrays

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();

    for i in 0..n {
        for j in 1..n{
            if nums[i]+ nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![] //Returning an empty array if no solution found!


}

fn main() {
    let nums = vec![1,2,3,4,5]; 
    let target  = 11;
    
    let answer = two_sum(nums, target);
    println!("{:?}",answer);
}
