extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const WINDOW_SIZE: u32 = 480;

const ROW_SQUARES: u32 = 32;
const MAX_SQUARES: u32 = ROW_SQUARES * ROW_SQUARES;
const SQUARE_SIZE: u32 = WINDOW_SIZE / ROW_SQUARES;

const STARTING_SQUARES: u32 = 5;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn gen_treat_coords(squares: &Vec<(i32, i32, Direction)>) -> (i32, i32) {
    let (mut x, mut y);

    loop {
        let square_index = rand::thread_rng().gen_range(0..MAX_SQUARES);

        x = (square_index % ROW_SQUARES) * SQUARE_SIZE;
        y = (square_index / ROW_SQUARES) * SQUARE_SIZE;

        let mut valid = true;
        for square in squares {
            if square.0 == x as i32 && square.1 == y as i32 {
                valid = false;
                break;
            }
        }

        if valid {
            break;
        }
    }

    (x as i32, y as i32)
}

fn new_square(squares: &mut Vec<(i32, i32, Direction)>) {
    squares.push(squares.last().unwrap().clone());

    let direction = squares[0].2;
    let last = squares.last_mut().unwrap();

    match direction {
        Direction::LEFT => {
            last.0 += SQUARE_SIZE as i32;
        }
        Direction::RIGHT => {
            last.0 -= SQUARE_SIZE as i32;
        }
        Direction::UP => {
            last.1 += SQUARE_SIZE as i32;
        }
        Direction::DOWN => {
            last.1 -= SQUARE_SIZE as i32;
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Snake", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut squares: Vec<(i32, i32, Direction)> = vec![(0, 0, Direction::RIGHT)];
    for _ in 0..STARTING_SQUARES {
        new_square(&mut squares);
    }
    let mut last_movement = Instant::now();
    let mut direction_queue: VecDeque<Direction> = VecDeque::new();
    let mut treat: (i32, i32) = gen_treat_coords(&squares);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Left | Keycode::A => {
                        direction_queue.push_back(Direction::LEFT);
                    }
                    Keycode::Right | Keycode::D => {
                        direction_queue.push_back(Direction::RIGHT);
                    }
                    Keycode::Up | Keycode::W => {
                        direction_queue.push_back(Direction::UP);
                    }
                    Keycode::Down | Keycode::S => {
                        direction_queue.push_back(Direction::DOWN);
                    }

                    _ => {}
                },
                _ => {}
            }
        }

        // movement is due
        if Instant::now() - last_movement >= Duration::from_millis(100) {
            let mut d = squares[0].2;

            while let Some(q_d) = direction_queue.pop_front() {
                if !(q_d == Direction::RIGHT && d == Direction::LEFT)
                    && !(q_d == Direction::LEFT && d == Direction::RIGHT)
                    && !(q_d == Direction::UP && d == Direction::DOWN)
                    && !(q_d == Direction::DOWN && d == Direction::UP)
                    && (q_d != d)
                {
                    d = q_d;
                    break;
                }
            }

            squares[0].2 = d;

            for i in (1..squares.len()).rev() {
                squares[i] = squares[i - 1]
            }

            match d {
                Direction::LEFT => {
                    squares[0].0 -= SQUARE_SIZE as i32;
                }
                Direction::RIGHT => {
                    squares[0].0 += SQUARE_SIZE as i32;
                }
                Direction::UP => {
                    squares[0].1 -= SQUARE_SIZE as i32;
                }
                Direction::DOWN => {
                    squares[0].1 += SQUARE_SIZE as i32;
                }
            }

            last_movement = Instant::now();
        }

        // treat eaten
        if squares[0].0 == treat.0 && squares[0].1 == treat.1 {
            new_square(&mut squares);

            treat = gen_treat_coords(&squares);
        }

        // crashed into self
        for square in &squares[1..squares.len()] {
            if squares[0].0 == square.0 && squares[0].1 == square.1 {
                break 'running;
            }
        }

        // crashed into border
        if squares[0].0 >= WINDOW_SIZE as i32
            || squares[0].0 < 0
            || squares[0].1 >= WINDOW_SIZE as i32
            || squares[0].1 < 0
        {
            break 'running;
        }

        // clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // draw snake
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for i in 0..squares.len() {
            canvas
                .fill_rect(sdl2::rect::Rect::new(
                    squares[i].0 as i32,
                    squares[i].1 as i32,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                ))
                .unwrap();
        }

        // draw treat
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas
            .fill_rect(sdl2::rect::Rect::new(
                treat.0 as i32,
                treat.1 as i32,
                SQUARE_SIZE,
                SQUARE_SIZE,
            ))
            .unwrap();

        canvas.present();
    }
}
