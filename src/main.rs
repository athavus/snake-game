extern crate pancurses;
use pancurses::{initscr, endwin, noecho, Input};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use std::thread::sleep;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeGame {
    snake: VecDeque<(i32, i32)>,
    direction: Direction,
    food: (i32, i32),
    width: i32,
    height: i32,
    score: i32,
}

impl SnakeGame {
    fn new(width: i32, height: i32) -> Self {
        let snake = VecDeque::from(vec![(width / 2, height / 2)]);
        SnakeGame {
            snake,
            direction: Direction::Right,
            food: (width / 4, height / 4),
            width,
            height,
            score: 0,
        }
    }

    fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        self.food = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));
    }

    fn advance_snake(&mut self) {
        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        if new_head == self.food {
            self.snake.push_front(new_head);
            self.score += 1;
            self.generate_food();
        } else {
            self.snake.push_front(new_head);
            self.snake.pop_back();
        }
    }

    fn check_collision(&self) -> bool {
        let head = self.snake.front().unwrap();
        head.0 < 0 || head.1 < 0 || head.0 >= self.width || head.1 >= self.height || self.snake.iter().skip(1).any(|&pos| pos == *head)
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if (self.direction == Direction::Up && new_direction != Direction::Down)
            || (self.direction == Direction::Down && new_direction != Direction::Up)
            || (self.direction == Direction::Left && new_direction != Direction::Right)
            || (self.direction == Direction::Right && new_direction != Direction::Left) {
            self.direction = new_direction;
        }
    }
}

fn main() {
    let width = 80;
    let height = 25;

    let window = initscr();
    window.timeout(100);
    noecho();

    let mut game = SnakeGame::new(width, height);
    let mut last_update = Instant::now();

    while !game.check_collision() {
        if let Some(input) = window.getch() {
            match input {
                Input::Character('a') => game.change_direction(Direction::Left),
                Input::Character('d') => game.change_direction(Direction::Right),
                Input::Character('w') => game.change_direction(Direction::Up),
                Input::Character('s') => game.change_direction(Direction::Down),
                _ => {}
            }
        }

        if last_update.elapsed() >= Duration::from_millis(100) {
            game.advance_snake();
            window.clear();

            for &(x, y) in &game.snake {
                window.mvaddch(y, x, 'O'); 
            }

            window.mvaddch(game.food.1, game.food.0, 'X'); 
            window.mvprintw(0, 0, &format!("Pontuação: {}", game.score));

            window.refresh();
            last_update = Instant::now();
        }

        sleep(Duration::from_millis(50));
    }

    window.clear();
    window.mvprintw(height / 2, width / 2 - 5, "Fim de Jogo!");
    window.refresh();
    sleep(Duration::from_secs(2));
    endwin();
}
