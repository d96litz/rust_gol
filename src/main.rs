use crossterm::{cursor, QueueableCommand};
#[allow(dead_code)]
use rand::{thread_rng, Rng};
use std::io;
use std::{thread, time};
extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn main() {
  let width = 100;
  let height = 100;
  let mut vec = gen_field(width, height);
  let stdin = stdin();
  let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

  write!(
    stdout,
    "{}{}Progress",
    termion::clear::All,
    termion::cursor::Goto(1, 1)
  )
  .unwrap();
  stdout.flush().unwrap();
  stdout = render_field(&vec, stdout, 2);

  let y_offset = 2;

  // for c in stdin.events() {
  //   let evt = c.unwrap();
  //   match evt {
  //     Event::Key(Key::Char('q')) => break,
  //     Event::Mouse(me) => {
  //       match me {
  //         MouseEvent::Press(_, x, y) => {
  //           // println!("{:?}", (x, y));
  //           if y < 3 {
  //             vec = run_day(&vec);
  //             stdout = render_field(&vec, stdout, y_offset);
  //           } else {
  //             let x_pos = x - 1;
  //             let y_pos = y - y_offset - 1;
  //             if x_pos <= width as u16 && y_pos <= height as u16 - 1 {
  //               // println!("pos: {:?}, {:?}", (x, y), x_pos as usize);
  //               vec = create_x(vec, (x - 1) as usize, y_pos as usize);
  //               stdout = render_field(&vec, stdout, y_offset);
  //             }
  //           }
  //         }
  //         _ => (),
  //       }
  //     }
  //     _ => {}
  //   }
  //   stdout.flush().unwrap();
  // }

  for _ in 0..200 {
    vec = run_day(&vec);
    stdout = render_field(&vec, stdout, y_offset);
    stdout.flush().unwrap();
    thread::sleep(time::Duration::from_millis(100));
  }
}

fn create_x(mut vec: Vec<Vec<bool>>, x_pos: usize, y_pos: usize) -> Vec<Vec<bool>> {
  vec[y_pos][x_pos] = true;

  vec
}

fn render_field(
  vec: &Vec<Vec<bool>>,
  mut term: MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
  y_offset: u16,
) -> MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>> {
  let mut output = "".to_owned();
  for (y_index, line) in vec.iter().enumerate() {
    output.push_str("│");
    for (x_index, element) in line.iter().enumerate() {
      if *element {
        write!(
          term,
          "{}o",
          termion::cursor::Goto(x_index as u16 + 1, y_index as u16 + 1 + y_offset)
        )
        .unwrap();
      } else {
        write!(
          term,
          "{} ",
          termion::cursor::Goto(x_index as u16 + 1, y_index as u16 + 1 + y_offset)
        )
        .unwrap();
      }
    }
  }
  term
}

fn gen_field(width: i32, height: i32) -> Vec<Vec<bool>> {
  (0..width)
    .map(|_| {
      (0..height)
        .map(|_| thread_rng().gen_bool(1.0 / 1.8))
        .collect()
    })
    .collect()
}

fn run_day(old_vec: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
  old_vec
    .iter()
    .enumerate()
    .map(|(y_index, line)| {
      line
        .iter()
        .enumerate()
        .map(|(x_index, _)| lives(old_vec, x_index, y_index))
        .collect()
    })
    .collect()
}

fn lives(vec: &Vec<Vec<bool>>, x_pos: usize, y_pos: usize) -> bool {
  let live_neighbors = live_neighbors(vec, x_pos, y_pos);
  if vec[x_pos][y_pos] {
    if (2..=3).contains(&live_neighbors) {
      return true;
    }
  } else {
    if live_neighbors == 3 {
      return true;
    }
  }

  false
}

fn live_neighbors(vec: &Vec<Vec<bool>>, x_pos: usize, y_pos: usize) -> usize {
  let max_y = vec[0].len() - 1;
  let max_x = vec.len() - 1;

  let mut neighbors = 0;
  for x in (x_pos as i32) - 1..=(x_pos as i32) + 1 {
    if x < 0 || x > max_x as i32 {
      continue;
    };
    for y in (y_pos as i32) - 1..=(y_pos as i32) + 1 {
      if y < 0 || y > max_y as i32 {
        continue;
      };
      if y == y_pos as i32 && x == x_pos as i32 {
        continue;
      };
      // println!("{:?}", (x, y));
      if vec[y as usize][x as usize] {
        neighbors += 1;
      }
    }
  }

  neighbors
}

// fn main() {
//   let mut vec = gen_field(10, 10);
//   let mut stdout = stdout();

//   for i in 1..100 {
//     // stdout.queue(cursor::SavePosition).unwrap();
//     stdout.write(render_field(&vec).as_bytes()).unwrap();
//     // stdout.write(format!("Here!!! {}", i).as_bytes());
//     // stdout.queue(cursor::RestorePosition).unwrap();
//     // stdout.flush().unwrap();
//     vec = run_day(&vec);
//     thread::sleep(time::Duration::from_millis(500));
//   }
//   vec = run_day(&vec);
//   println!();
//   println!();
//   render_field(&vec);
// }
