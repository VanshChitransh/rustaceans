fn main(){
    let numbers = vec![1,2,3,4,5];
    let (max_number, min_number, even_count) = analyze_vector(&numbers);
    println!("Max -> {}, Min -> {}, even_Count -> {}", max_number, min_number, even_count);
}

fn analyze_vector(numbers: &Vec<i32>) -> (&i32, &i32, usize){
    let mut max: &i32 = &numbers[0]; 
    let mut min: &i32 = &numbers[0]; 
    let mut cnt: usize = 0;
    for i in 0..numbers.len(){
        if numbers[i] > *max{
            max = &numbers[i];
        }
        if numbers[i] < *min{
            min = &numbers[i];
        }
        if numbers[i] % 2 == 0{
            cnt += 1;
        }
    }
    (max, min, cnt)
}