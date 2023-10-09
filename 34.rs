use std::cmp;

impl Solution {
    /// start_idx is inclusive, assumed 0 <= start_idx is < end_idx
    /// end_idx is exclusive, assumed end_idx is <= nums.len()
    /// returns the index of first `target` found in nums
    fn binser(nums: &Vec<i32>, target: i32) -> i32 {
        let mut end_idx = nums.len();
        let mut start_idx = 0_usize;
        let mut mid_idx = (start_idx + end_idx) / 2;
        let mut num = nums[mid_idx];

        while 0 <= mid_idx && mid_idx < end_idx && start_idx < end_idx && num != target {
            if num == target {
                return mid_idx as i32;
            }

            if num < target {
                start_idx = mid_idx + 1;
            } else {
                end_idx = mid_idx;
            }

            mid_idx = (start_idx + end_idx) / 2;
            if mid_idx < 0 || mid_idx >= nums.len() {
                return -1;
            }
   
            num = nums[mid_idx];
        }

        return if num == target {
            mid_idx as i32
        } else {
            -1
        };
    }

    fn find_first_idx(nums: &Vec<i32>, binser_idx: i32, target: i32) -> i32 {
        if binser_idx == 0 {
            return 0;
        }

        let mut start_idx = 0;
        let mut end_idx = binser_idx;
        let mut mid_idx = (start_idx + end_idx) / 2;
        let mut num = nums[mid_idx as usize];

        while 0 <= mid_idx && mid_idx < end_idx && start_idx < end_idx {
            ///println!("{}\t{}\t{}\t{}", start_idx, end_idx, mid_idx, num);
            if num == target {
                end_idx = mid_idx;
            } else {
                start_idx = mid_idx + 1;
            }
     
            mid_idx = (start_idx + end_idx) / 2;
            num = nums[mid_idx as usize];
        }
        //println!("{}\t{}\t{}\t{}", start_idx, end_idx, mid_idx, num);

        return start_idx;
    }

    fn find_last_idx(nums: &Vec<i32>, binser_idx: i32, target: i32) -> i32 {
        let final_idx = nums.len() as i32 - 1;

        if binser_idx == final_idx {
            return final_idx;
        }

        let mut start_idx = binser_idx;
        let mut end_idx = final_idx;
        let mut mid_idx = (start_idx + end_idx) / 2;
        let mut num = nums[mid_idx as usize];

        while 0 <= mid_idx && mid_idx < end_idx && start_idx < end_idx {
            //println!("{}\t{}\t{}\t{}", start_idx, end_idx, mid_idx, num);
            if num == target {
                start_idx = mid_idx + 1;
            } else {
                end_idx = mid_idx;
            }

            // make sure end_idx isn't stuck when end_idx is not on target
            if nums[end_idx as usize] != target {
                end_idx -= 1;
            }
     
            mid_idx = (start_idx + end_idx) / 2;
            num = nums[mid_idx as usize];
        }
        //println!("{}\t{}\t{}\t{}", start_idx, end_idx, mid_idx, num);

        // end_idx is always offset-ed by one if nums[end_idx] is not the same as target
        return if nums[end_idx as usize] != target {
            end_idx - 1
        } else {
            end_idx
        };
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let binser_idx = Solution::binser(&nums, target);
        if binser_idx == -1 {
            return vec![-1, -1];
        }
        let first_idx = Solution::find_first_idx(&nums, binser_idx, target);
        //println!("--------------");
        let last_idx = Solution::find_last_idx(&nums, binser_idx, target);

        return vec![first_idx, last_idx];
    }
}
