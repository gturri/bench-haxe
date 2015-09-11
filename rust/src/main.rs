fn main() {
  let mut v : Vec<Vec<f32>> = vec![
               vec![3f32],
               vec![7f32, 4f32],
               vec![2f32, 4f32, 6f32],
               vec![8f32, 5f32, 9f32, 3f32]];

  let mut sum = 0f32;
  for _ in 0..10000000 {
     sum += solve(&mut v);
  }
  println!("{}", sum);
}

fn solve(t: &mut Vec<Vec<f32>>) -> f32 {
  for idx_line in 1..t.len() {
    for idx_col in 0..t[idx_line].len() {
      if idx_col == 0 {
        t[idx_line][idx_col] += t[idx_line-1][0];
      } else if idx_col == t[idx_line].len() - 1 {
        t[idx_line][idx_col] += t[idx_line-1][idx_col - 1];
      } else {
        t[idx_line][idx_col] += my_max(t[idx_line-1][idx_col-1], t[idx_line-1][idx_col]);
      }
    }
  }

  let mut maxi = 1.0f32;
  for item in t[t.len()-1].iter() {
    maxi = my_max(maxi, *item);
  }
  maxi
}

fn my_max(a: f32, b: f32) -> f32 {
  if a > b { a } else { b }
}
