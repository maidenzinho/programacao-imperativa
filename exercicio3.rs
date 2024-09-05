fn main(){
    let nums = vec![5,6,8,9,3,2,7,1];
    let mut men: i32 = nums[1];
    
    for n in nums{
        if men > n{
            men = n;
        }
    }
    println!("Maior número é {}", men);
    
    }