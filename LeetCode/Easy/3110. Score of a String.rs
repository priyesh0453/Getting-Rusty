struct Solution;

impl Solution 
{
    pub fn score_of_string(s: String) -> i32 
    {
        let chars: Vec<char> = s.chars().collect();
        let mut stringScore: i32 = 0;

        for i in 0..chars.len() - 1 
        {
            stringScore += ((chars[i] as i32) - (chars[i + 1] as i32)).abs();
        }

        return stringScore;
    }
}

fn main() 
{
    let input_string = String::from("hello");
    let score = Solution::score_of_string(input_string);
    
    println!("Score of the string: {}", score);
}