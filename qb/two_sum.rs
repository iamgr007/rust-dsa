// two sum problem

fn main() {
    let arr = [2, 7, 11, 15];
    let N = arr.len();
  
    for i in 0..N {
        for j in 0..N  {
            if arr[i] + arr[j] == 9 {
                println!("{}{}", i, j);
            }
        }
    }
}
