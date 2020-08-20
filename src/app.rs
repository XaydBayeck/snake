use crate::{collision::*, consts, render::Render, Direction, Fruit, GameStatus, Snake, Wall};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::*;
use piston::{RenderArgs, UpdateArgs, WindowSettings};
use rand::Rng;

/// 应用程序主体结构体
#[derive(Clone)]
pub struct App {
    pub game_status: GameStatus, // 游戏状态机
    pub circus: [u32; 2],        // 移动空间
    update_time: f64,            // 记录一次更新后的时间
    pub score: u32,              // 记录玩家的分数
    pub board_wall: Wall,        // 边界
    pub walls: Vec<Wall>,        // 随机生成的墙壁
    pub fruit: Fruit,            // 食物
    pub snake: Snake,            // 蛇蛇
}

impl App {
    /// 建立新的App实例
    pub fn new(horizontal_block_num: u32, vertical_block_num: u32) -> Self {
        let circus = [horizontal_block_num, vertical_block_num];

        let mut walls = Vec::<Wall>::new();
        let walls_num = rand::thread_rng().gen_range(1, 10);
        for _ in 0..walls_num {
            let brick_num = rand::thread_rng().gen_range(5, 10);

            walls.push(Wall::randnew(Some(brick_num), &circus));
        }

        App {
            game_status: GameStatus::GAMMING,
            circus,
            update_time: 0.0,
            score: 0,
            board_wall: Wall::board_wall(&circus),
            walls,
            fruit: Fruit::randnew(horizontal_block_num, vertical_block_num),
            snake: Snake::new(horizontal_block_num, vertical_block_num),
        }
    }

    /// 理论计算更新主函数
    fn update(&mut self, args: &UpdateArgs) {
        match self.game_status {
            // 游戏中
            GameStatus::GAMMING => {
                // 积累下一次更新的时间
                self.update_time += args.dt;

                // 我们以固定的时间间隔更新游戏的逻辑
                if self.update_time >= (1.0 / self.snake.velocity) {
                    // 解锁方向
                    self.snake.direction_lock = false;

                    // 碰撞检测
                    match self.is_collision() {
                        Collited::WithFruit => self.growth_action(),
                        Collited::NoCollision => (),
                        _ => self.game_status = GameStatus::GAMEOVER,
                    }

                    // 移动蛇
                    self.snake.moving();

                    // 初始化时间
                    self.update_time = 0.0;
                }
            }
            // 重启
            GameStatus::RESTART => {
                let pristine = App::new(self.circus[0], self.circus[1]);
                *self = pristine;
                return;
            }
            // 如果游戏结束或暂停就不继续更新了
            _ => (),
        }
    }

    // 蛇身加长的操作方法
    fn growth_action(&mut self) {
        // 调用蛇的方法
        self.snake.growth_action();
        // 仅选择一个随机位置，也可能在蛇上
        self.fruit = Fruit::randnew(self.circus[0], self.circus[1]);
        // 增加分数
        self.score += 1;
    }

    // 碰撞检测
    fn is_collision(&mut self) -> Collited {
        // 碰撞到了身体？
        if self.snake.is_colliting_with_self() {
            return Collited::WithSnake;
        }

        // 碰撞到了边界？
        if self.board_wall.is_collited_by_block(&self.snake.head) == Collited::WithWall {
            return Collited::WithWall;
        }

        // 碰撞到了墙壁？
        for wall in self.walls.iter() {
            if wall.is_collited_by_block(&self.snake.head) == Collited::WithWall {
                return Collited::WithWall;
            }
        }

        // 碰撞到了食物？
        if self.fruit.is_collited_by_block(&self.snake.head) == Collited::WithFruit {
            return Collited::WithFruit;
        }

        Collited::NoCollision
    }

    // 按键判定
    fn press(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(key) => self.key_press(key),
            _ => {}
        }
    }

    fn key_press(&mut self, key: Key) {
        if self.snake.direction_lock == false {
            self.snake.direction_lock = true;
            match key {
                Key::Up if self.snake.direction != Direction::Down => {
                    self.snake.direction = Direction::Up;
                }
                Key::Down if self.snake.direction != Direction::Up => {
                    self.snake.direction = Direction::Down;
                }
                Key::Left if self.snake.direction != Direction::Right => {
                    self.snake.direction = Direction::Left;
                }
                Key::Right if self.snake.direction != Direction::Left => {
                    self.snake.direction = Direction::Right;
                }
                Key::Space => {
                    self.game_status = match self.game_status {
                        GameStatus::GAMMING => GameStatus::TIMEOUT,
                        _ => GameStatus::RESTART,
                    }
                }
                Key::Return => {
                    self.game_status = match self.game_status {
                        GameStatus::TIMEOUT => GameStatus::GAMMING,
                        _ => self.game_status.clone(),
                    }
                }
                _ => {}
            }
        }
    }

    /// 运行程序
    pub fn run(&mut self) {
        // piston标准结构，与渲染有关
        let opengl = OpenGL::V3_2;

        // 创建一个Glutin窗口
        let mut window: Window = WindowSettings::new("snakes", [640, 480])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

        // piston标准结构，与渲染有关
        let mut gl = GlGraphics::new(opengl);

        // 创建一个新的事件并设置更新频率
        let ref mut events = Events::new(EventSettings::new());
        events.set_ups(60);

        // piston引擎的主要循环，是以迭代器的形式实现的
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, gl| {
                    self.render(&self.circus, &args, gl, c);
                });
            }
            if let Some(args) = e.update_args() {
                self.update(&args);
            }
            if let Some(button) = e.press_args() {
                self.press(&button);
            }
        }
    }
}
