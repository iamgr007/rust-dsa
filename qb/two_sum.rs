// two sum problem

fn main() {
}fn main() {
  let arr = [2, 7, 11, 15];
    
    for i in 0..arr.len() {
        for j in 0..arr.len()  {
            if arr[i] + arr[j] == 9 {
                println!("{}{}", i, j);
            }
        }
    }
}
