fn main() {
  let mut v : Vec<Vec<f32>> = vec![
               vec![3f32],
               vec![7f32, 4f32],
               vec![2f32, 4f32, 6f32],
               vec![8f32, 5f32, 9f32, 3f32]];

  let mut sum = 0f32;
  for i in 0..10000000 {
     sum += solve(&mut v);
  }
  println!("{}", sum);
}

fn solve(t: &mut Vec<Vec<f32>>) -> f32 {
  for idxLine in 1..t.len() {
    for idxCol in 0..t[idxLine].len() {
      if idxCol == 0 {
        t[idxLine][idxCol] += t[idxLine-1][0];
      } else if idxCol == t[idxLine].len() - 1 {
        t[idxLine][idxCol] += t[idxLine-1][idxCol - 1];
      } else {
        t[idxLine][idxCol] += my_max(t[idxLine-1][idxCol-1], t[idxLine-1][idxCol]);
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
