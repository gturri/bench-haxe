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
  let mut prev_line = &vec![];
  for (idx_line, cur_line) in t.into_iter().enumerate() {
    if idx_line != 0 {
      update_row(&prev_line, cur_line);
    }
    prev_line = cur_line;
  }

  max_in_row(&prev_line)
}

fn max_in_row(row: &Vec<f32>) -> f32 {
  let mut maxi = -1.0f32;
  for item in row {
    maxi = my_max(maxi, *item);
  }
  maxi
}

fn update_row(prev_line: &Vec<f32>, cur_line: &mut Vec<f32>){
  for idx_col in 0..cur_line.len(){
    if idx_col == 0 {
      cur_line[idx_col] += prev_line[0];
    } else if idx_col == cur_line.len() - 1 {
      cur_line[idx_col] += prev_line[idx_col - 1];
    } else {
      cur_line[idx_col] += my_max(prev_line[idx_col-1], prev_line[idx_col]);
    }
  }
}

fn my_max(a: f32, b: f32) -> f32 {
  if a > b { a } else { b }
}
