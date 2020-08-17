use glutin_window::GlutinWindow as Window;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use rand::Rng;

// 包含可以在游戏中使用的颜色
pub mod game_colors {
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const LIGHTBLUE: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const PINK: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
    pub const ANGEL: [f32; 4] = [0.5, 0.5, 1.0, 0.5];
    pub const GREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
}

#[derive(Debug, Clone, PartialEq)]
struct Block {
    pos_x: i32,
    pos_y: i32,
}

impl Block {
    pub fn randnew(horizontal_block_num: u32, vertical_block_num: u32) -> Self {
        Block {
            pos_x: rand::thread_rng().gen_range(1, (horizontal_block_num - 1) as i32),
            pos_y: rand::thread_rng().gen_range(1, (vertical_block_num - 1) as i32),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Collision {
    WithFruit,
    WithSnake,
    WithBorder,
    NoCollision,
}

// App结构体
#[derive(Clone)]
pub struct App {
    velocity: f64,          // 移动速度
    direction: Direction,   // 移动方向
    direction_lock: bool,   // 如果不想改变方向，则设置为true
    growth_flag: bool,      // 如果body_block数量增加，则设置为true
    gameover_flag: bool,    // 如果游戏结束，则设置为true
    restart_flag: bool,     // 如果游戏重启，则设置为true
    head_block: Block,      // 蛇蛇在piston中的头部方块
    body_block: Vec<Block>, // 蛇蛇在piston中的身体方块
    fruit_block: Block,     // 食物在pistonh中的方块
    circus: [u32; 2],       // 移动空间
    update_time: f64,       // 记录一次更新后的时间
    score: u32,             // 记录玩家的分数
}

// App方法实现
impl App {
    //新建方法
    pub fn new(horizontal_block_num: u32, vertical_block_num: u32) -> Self {
        let center_x = ((horizontal_block_num as f64) * 0.5) as i32;
        let center_y = ((vertical_block_num as f64) * 0.5) as i32;

        App {
            velocity: 6.0,
            circus: [horizontal_block_num, vertical_block_num],
            head_block: Block {
                pos_x: center_x,
                pos_y: center_y,
            },
            body_block: vec![
                Block {
                    pos_x: center_x + 1,
                    pos_y: center_y,
                },
                Block {
                    pos_x: center_x + 2,
                    pos_y: center_y,
                },
                Block {
                    pos_x: center_x + 3,
                    pos_y: center_y,
                },
                Block {
                    pos_x: center_x + 4,
                    pos_y: center_y,
                },
            ],
            fruit_block: Block::randnew(horizontal_block_num, vertical_block_num),
            direction: Direction::Left,
            update_time: 0.0,
            direction_lock: false,
            growth_flag: false,
            gameover_flag: false,
            restart_flag: false,
            score: 0,
        }
    }

    // 渲染方法的主函数
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, glyp_cache: &mut GlyphCache) {
        use graphics::*;

        // 只绘制game over画面
        if self.gameover_flag {
            self.gameover_render(args, gl, glyp_cache);
            return;
        }

        // 绘制视图
        gl.draw(args.viewport(), |c, gl| {
            // 清空屏幕
            clear(game_colors::BLACK, gl);

            // 绘制蛇头
            rectangle(
                game_colors::RED,
                self.renderable_rect(&self.head_block, args),
                c.transform,
                gl,
            );

            // 绘制食物
            rectangle(
                game_colors::GREEN,
                self.renderable_rect(&self.fruit_block, args),
                c.transform,
                gl,
            );

            // 绘制一个游戏的边框
            let vertical_line_radius = (args.window_size[0] as f64) / (self.circus[0] as f64) * 0.5;
            let horizontal_line_radius =
                (args.window_size[1] as f64) / (self.circus[1] as f64) * 0.5;

            line(
                game_colors::LIGHTBLUE,
                horizontal_line_radius,
                [0.0, 0.0, 0.0, args.window_size[0] as f64],
                c.transform,
                gl,
            );
            line(
                game_colors::LIGHTBLUE,
                vertical_line_radius,
                [
                    args.window_size[0] as f64,
                    0.0,
                    args.window_size[0] as f64,
                    args.window_size[1] as f64,
                ],
                c.transform,
                gl,
            );
            line(
                game_colors::LIGHTBLUE,
                horizontal_line_radius,
                [0.0, 0.0, args.window_size[0] as f64, 0.0],
                c.transform,
                gl,
            );
            line(
                game_colors::LIGHTBLUE,
                horizontal_line_radius,
                [
                    0.0,
                    args.window_size[1] as f64,
                    args.window_size[0] as f64,
                    args.window_size[1] as f64,
                ],
                c.transform,
                gl,
            );

            // 绘制蛇身
            for block in self.body_block.iter() {
                rectangle(
                    game_colors::WHITE,
                    self.renderable_rect(block, args),
                    c.transform,
                    gl,
                );
            }

            // 绘制分数
            text(
                game_colors::WHITE,
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
        use graphics::*;

        // 绘制视图
        gl.draw(args.viewport(), |c, gl| {
            // 清空屏幕
            clear(game_colors::BLACK, gl);

            // 显示游戏结束和分数
            text(
                color::WHITE,
                15,
                format!("Game over! Press Space to restart, Escape to quit!").as_str(),
                glyp_cache,
                c.transform.trans(10.0, 40.0),
                gl,
            )
            .unwrap();

            text(
                color::WHITE,
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

    // 渲染过程中Block的尺度变换
    fn renderable_rect(&self, block: &Block, args: &RenderArgs) -> [f64; 4] {
        use graphics::*;

        let block_size_x = args.window_size[0] / (self.circus[0] as f64);
        let block_size_y = args.window_size[1] / (self.circus[1] as f64);
        let window_pos_x = (block.pos_x as f64) * block_size_x;
        let window_pos_y = (block.pos_y as f64) * block_size_y;
        rectangle::rectangle_by_corners(
            window_pos_x - block_size_x * 0.5,
            window_pos_y - block_size_y * 0.5,
            window_pos_x + block_size_x * 0.5,
            window_pos_y + block_size_y * 0.5,
        )
    }

    // 理论计算更新主函数
    fn update(&mut self, args: &UpdateArgs) {
        // 重启
        if self.restart_flag {
            let pristine = App::new(self.circus[0], self.circus[1]);
            *self = pristine;
            return;
        }

        // 积累下一次更新的时间
        self.update_time += args.dt;

        // 我们以固定间隔更新游戏逻辑
        if self.update_time >= (1.0 / self.velocity) {
            // 解锁方向
            self.direction_lock = false;

            // 查看碰撞
            match self.is_collision() {
                Collision::WithFruit => self.growth_action(),
                Collision::NoCollision => (),
                Collision::WithBorder => self.gameover_flag = true,
                Collision::WithSnake => self.gameover_flag = true,
            }

            if self.gameover_flag {
                return;
            }

            // 坐标移动
            let (x, y) = match self.direction {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Right => (1, 0),
                Direction::Left => (-1, 0),
            };

            // 克隆当前的坐标，会成为身体的一部分
            let mut pre_block = self.head_block.clone();

            // 更新蛇头坐标
            self.head_block.pos_x += x;
            self.head_block.pos_y += y;

            // 通过将蛇体的当前块推到新向量来“移动”蛇
            let mut blocks = Vec::new();
            for block in self.body_block.iter_mut() {
                blocks.push(pre_block);
                pre_block = block.clone();
            }

            // 如果设置了增长标志，请不要浪费任何块。
            if self.growth_flag {
                blocks.push(pre_block);
                self.growth_flag = false;
            }

            // 分配新的身体
            self.body_block = blocks;
            // 初始化时间
            self.update_time = 0.0;
        }
    }

    // 蛇身加长的操作方法
    pub fn growth_action(&mut self) {
        // 设置 growth_flag
        self.growth_flag = true;
        // 增加移动速度
        self.velocity += 0.01;
        // 仅选择一个随机位置，也可能在蛇上
        self.fruit_block = Block::randnew(self.circus[0], self.circus[1]);
        // 增加分数
        self.score += 1;
    }

    // 碰撞判断
    fn is_collision(&mut self) -> Collision {
        // 碰撞到了身体？
        for block in self.body_block.iter() {
            if self.head_block == *block {
                //println!("Collid with body");
                //println!("head:{:?},body:{:?}", self.head_block, block);
                return Collision::WithSnake;
            }
        }

        // 碰撞到了边界？
        if self.head_block.pos_x <= 0
            || self.head_block.pos_x >= self.circus[0] as i32
            || self.head_block.pos_y <= 0
            || self.head_block.pos_y >= self.circus[1] as i32
        {
            //println!("Collid with border");
            return Collision::WithBorder;
        }

        // 碰撞到了食物？
        if self.head_block == self.fruit_block {
            //println!("Collid with body");
            return Collision::WithFruit;
        }

        Collision::NoCollision
    }

    // 按键判定
    pub fn press(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(key) => self.key_press(key),
            _ => {}
        }
    }

    pub fn key_press(&mut self, key: Key) {
        if self.direction_lock == false {
            self.direction_lock = true;
            match key {
                Key::Up if self.direction != Direction::Down => {
                    self.direction = Direction::Up;
                }
                Key::Down if self.direction != Direction::Up => {
                    self.direction = Direction::Down;
                }
                Key::Left if self.direction != Direction::Right => {
                    self.direction = Direction::Left;
                }
                Key::Right if self.direction != Direction::Left => {
                    self.direction = Direction::Right;
                }
                Key::Space => {
                    self.restart_flag = true;
                    return;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    // piston标准结构，与渲染有关
    let opengl = OpenGL::V3_2;

    // 创建一个Glutin窗口
    let mut window: Window = WindowSettings::new("snakes", [640, 480])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    // 这里的App是主要结构体，构建一个理论尺寸为(80, 60)的结构体
    let mut app = App::new(80, 60);

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
            app.render(&args, &mut gl, &mut glyph_cache);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(button) = e.press_args() {
            app.press(&button);
        }
    }
}
