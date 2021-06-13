#[allow(dead_code)]

// fn main() {
//   let imgx = 800;
//   let imgy = 800;

//   let pixels: Vec<Vec<bool>> = (0..imgx)
//     .map(|_| (0..imgy).map(|x| false).collect())
//     .collect();

//   println!("{:?}", pixels);

//   println!("Hello, world!");
// }
use rand::{thread_rng, Rng};
use std::io::{Write, stdout};
use std::{thread, time};
use crossterm::{QueueableCommand, cursor};

fn main() {
  let mut vec = gen_field(10, 10);
  let mut stdout = stdout();

  for i in 1..100 {
    // stdout.queue(cursor::SavePosition).unwrap();
    stdout.write(render_field(&vec).as_bytes()).unwrap();
    // stdout.write(format!("Here!!! {}", i).as_bytes());
    // stdout.queue(cursor::RestorePosition).unwrap();
    // stdout.flush().unwrap();
    vec = run_day(&vec);
    thread::sleep(time::Duration::from_millis(500));
  }
  vec = run_day(&vec);
  println!();
  println!();
  render_field(&vec);
}

fn render_field(vec: &Vec<Vec<bool>>) -> String {
  let mut output = "".to_owned();
  for (y_index, line) in vec.iter().enumerate() {
    output.push_str("│");
    for (x_index, element) in line.iter().enumerate() {
      if *element {
        output.push_str("x");
      } else {
        output.push_str(" ");
      }
      // live_neighbors(&vec, x_index as i32, y_index as i32);
      output.push_str(" │ ");
    }
    output.push_str("\n────────────────────────────────────────");
    // print!("              ");
    // for (x_index, _) in line.iter().enumerate() {
    //   print!("{}  ", live_neighbors(&vec, x_index, y_index));
    // }
    output.push_str("\n");
  }
  output
}


fn gen_field(width: i32, height: i32) -> Vec<Vec<bool>> {
  (0..width).map(|_| (0..height).map(|_| thread_rng().gen_bool(1.0/2.0)).collect()).collect()
}


fn run_day(old_vec: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
  old_vec.iter().enumerate().map(|(y_index, line)| line.iter().enumerate().map(|(x_index, _)| lives(old_vec, x_index, y_index) ).collect()).collect()
}

fn lives(vec: &Vec<Vec<bool>>, x_pos: usize, y_pos: usize) -> bool {
  let live_neighbors = live_neighbors(vec, x_pos, y_pos);
  if vec[x_pos][y_pos] {
    if (2..=3).contains(&live_neighbors) { return true }
  } else {
    if live_neighbors == 3 { return true }
  }

  false
}

fn live_neighbors(vec: &Vec<Vec<bool>>, x_pos: usize, y_pos: usize) -> usize {
  let max_y = vec[0].len() - 1;
  let max_x = vec.len() - 1;

  let mut neighbors = 0;
  for x in (x_pos as i32)-1..=(x_pos as i32)+1 {
    if x < 0 || x > max_x as i32 { continue };
    for y in (y_pos as i32)-1..=(y_pos as i32)+1 {
      if y < 0 || y > max_y as i32{ continue };
      if y == y_pos as i32 && x == x_pos as i32 { continue };
      // println!("{:?}", (x, y));
      if vec[y as usize][x as usize] {
        neighbors += 1;
      }
    }
  }

  neighbors
}