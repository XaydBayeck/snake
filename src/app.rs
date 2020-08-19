use crate::{collision::*, consts, render::Render, Direction, Fruit, Snake, Wall};
use glutin_window::GlutinWindow as Window;
use graphics::{clear, text, Transformed};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::*;
use piston::{RenderArgs, UpdateArgs, WindowSettings};
use rand::Rng;

/// 应用程序主体结构体
#[derive(Clone)]
pub struct App {
    gameover_flag: bool, // 如果游戏结束，则设置为true
    restart_flag: bool,  // 如果游戏重启，则设置为true
    circus: [u32; 2],    // 移动空间
    update_time: f64,    // 记录一次更新后的时间
    score: u32,          // 记录玩家的分数
    board_wall: Wall,    // 边界
    walls: Vec<Wall>,    // 随机生成的墙壁
    fruit: Fruit,        // 食物
    snake: Snake,        // 蛇蛇
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
            gameover_flag: false,
            restart_flag: false,
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
        // 重启
        if self.restart_flag {
            let pristine = App::new(self.circus[0], self.circus[1]);
            *self = pristine;
            return;
        }

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
                Collited::WithWall => self.gameover_flag = true,
                Collited::WithSnake => self.gameover_flag = true,
            }

            // 如果游戏结束就不继续更新了
            if self.gameover_flag {
                return;
            }

            // 移动蛇
            self.snake.moving();

            // 初始化时间
            self.update_time = 0.0;
        }
    }

    /// 渲染方法的主函数
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, glyp_cache: &mut GlyphCache) {
        // 只绘制game over画面
        if self.gameover_flag {
            self.gameover_render(args, gl, glyp_cache);
            return;
        }

        // 绘制视图
        gl.draw(args.viewport(), |c, gl| {
            // 清空屏幕
            clear(consts::BLACK, gl);

            // 绘制蛇
            self.snake.render(&self.circus, args, gl, c);

            // 绘制边框
            self.board_wall.render(&self.circus, args, gl, c);

            // 绘制墙壁
            for wall in self.walls.iter() {
                wall.render(&self.circus, args, gl, c);
            }

            // 绘制食物
            self.fruit.render(&self.circus, args, gl, c);

            // 绘制分数
            text(
                consts::WHITE,
                15,
                format!("Your score is {}", self.score).as_str(),
                glyp_cache,
                c.transform.trans(10.0, 20.0),
                gl,
            )
            .unwrap();
        })
    }

    // 结束画面的渲染方法
    fn gameover_render(&self, args: &RenderArgs, gl: &mut GlGraphics, glyp_cache: &mut GlyphCache) {
        // 绘制视图
        gl.draw(args.viewport(), |c, gl| {
            // 清空屏幕
            clear(consts::BLACK, gl);

            // 显示游戏结束和分数
            text(
                consts::WHITE,
                15,
                format!("Game over! Press Space to restart, Escape to quit!").as_str(),
                glyp_cache,
                c.transform.trans(10.0, 40.0),
                gl,
            )
            .unwrap();

            text(
                consts::WHITE,
                15,
                format!("Your score is {}", self.score).as_str(),
                glyp_cache,
                c.transform.trans(10.0, 20.0),
                gl,
            )
            .unwrap();
            return;
        });
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
                    self.restart_flag = true;
                    return;
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

        // 为了能够渲染文字，需要读取字体缓存
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let mut glyph_cache = GlyphCache::new("assets/Roboto-Regular.ttf", (), texture_settings)
            .expect("Error unwrapping fonts");

        // 创建一个新的事件并设置更新频率
        let ref mut events = Events::new(EventSettings::new());
        events.set_ups(60);

        println!("Start loop!");
        // piston引擎的主要循环，是以迭代器的形式实现的
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                self.render(&args, &mut gl, &mut glyph_cache);
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
