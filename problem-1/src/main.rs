use itertools::Itertools;

fn multiples_of_f(f: i32, max: i32) -> Vec<i32> {
    let mut ms = Vec::new();
    let max_i = ((max-1) / f) as i32;
    for i in 1..max_i+1 { ms.push(f * i); }
    return ms;
}

fn multiples_of_fs(fs: Vec<i32>, max: i32) -> Vec<i32> {
    let mut ms = Vec::new();
    for f in fs { ms.extend(multiples_of_f(f, max)); }
    return ms.into_iter().unique().collect();
}

fn sum(slice: &[i32]) -> i32 {
    match slice[..] {
        [] => { return 0; }
        _ => { return slice[0] + sum(&slice[1..]); }
    }
}

fn main() {
    let multiples = multiples_of_fs(vec![3, 5], 1000);
    println!("{:?}", multiples);
    println!("Sum: {}", sum(&multiples[..]));
}