use app::App;
use collision::{Collision, Collited};
use rand::Rng;
use std::cmp::min;

/// 应用程序主体
mod app;
/// 碰撞系统
mod collision;
/// 包含会用到的预设常量
mod consts;
/// 渲染系统
mod render;

fn main() {
    // 这里的App是主要结构体，构建一个理论尺寸为(80, 60)的结构体
    let mut app = App::new(80, 60);

    // 运行程序
    app.run();
}

/// 蛇的移动方向枚举
#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// 基础的游戏实体
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// x坐标
    pos_x: i32,
    /// y坐标
    pos_y: i32,
    /// 被碰撞时返回的碰撞检测类型
    collited: Collited,
    /// 实体的颜色
    color: [f32; 4],
}

impl Block {
    /// 创建新的Block实例
    pub fn new(_pos_x: i32, _pos_y: i32, _collited: Collited, _color: [f32; 4]) -> Self {
        Block {
            pos_x: _pos_x,
            pos_y: _pos_y,
            collited: _collited,
            color: _color,
        }
    }
}

/// 墙壁实体
#[derive(Clone, Debug)]
pub struct Wall {
    bricks: Vec<Block>,
}

impl Wall {
    /// 边框
    /// `circus: &[u32; 2]` 是玩家的可移动范围
    pub fn board_wall(circus: &[u32; 2]) -> Self {
        let width = circus[0] as i32;
        let height = circus[1] as i32;

        let wall_color = consts::LIGHTBLUE;
        let mut bricks = Vec::<Block>::new();

        // 添加上下俩个边框
        for i in 0..width {
            let brick_up = Block::new(i, height, Collited::WithWall, wall_color);
            let brick_down = Block::new(i, 0, Collited::WithWall, wall_color);

            bricks.push(brick_up);
            bricks.push(brick_down);
        }

        // 添加左右俩个边框
        for i in 0..height {
            let brick_left = Block::new(0, i, Collited::WithWall, wall_color);
            let brick_right = Block::new(width, i, Collited::WithWall, wall_color);

            bricks.push(brick_left);
            bricks.push(brick_right);
        }

        Wall { bricks }
    }

    /// 创建一堵随机的墙
    pub fn randnew(brick_num: Option<u32>, circus: &[u32; 2]) -> Self {
        let x = rand::thread_rng().gen_range(1, (circus[0] - 1) as i32);
        let y = rand::thread_rng().gen_range(1, (circus[1] - 1) as i32);

        let mut bricks = Vec::<Block>::new();

        let n = match brick_num {
            Some(num) => num,
            None => rand::thread_rng().gen_range(1, min(circus[0], circus[1]) / 2),
        };

        for i in 0..n {
            let d = match rand::thread_rng().gen_bool(0.5) {
                true => 1i32,
                false => -1i32,
            };
            let (dx, dy) = match rand::thread_rng().gen_bool(0.5) {
                true => (d, 0),
                false => (0, d),
            };

            let x_pre = match bricks.get(i as usize) {
                Some(brick) => brick.pos_x,
                None => x,
            };

            let y_pre = match bricks.get(i as usize) {
                Some(brick) => brick.pos_y,
                None => y,
            };

            let brick = Block::new(
                x_pre + dx * (i as i32),
                y_pre + dy * (i as i32),
                Collited::WithWall,
                consts::LIGHTBLUE,
            );

            bricks.push(brick);
        }

        //println!("{:?}/n", bricks);

        Wall { bricks }
    }
}

/// 食物实体
#[derive(Clone)]
pub struct Fruit {
    block: Block,
}

impl Fruit {
    /// 创建位置随机的Fruit实例
    pub fn randnew(horizontal_block_num: u32, vertical_block_num: u32) -> Self {
        Fruit {
            block: Block {
                pos_x: rand::thread_rng().gen_range(1, (horizontal_block_num - 1) as i32),
                pos_y: rand::thread_rng().gen_range(1, (vertical_block_num - 1) as i32),
                collited: Collited::WithFruit,
                color: consts::GREEN,
            },
        }
    }
}

/// 蛇的实体
#[derive(Clone)]
pub struct Snake {
    head: Block,
    body: Vec<Block>,
    velocity: f64,
    direction: Direction,
    direction_lock: bool,
    growth_flag: bool,
}

impl Snake {
    // 在移动范围中央创建新的蛇蛇
    pub fn new(horizontal_block_num: u32, vertical_block_num: u32) -> Self {
        let center_x = ((horizontal_block_num as f64) * 0.5) as i32;
        let center_y = ((vertical_block_num as f64) * 0.5) as i32;

        Snake {
            head: Block {
                pos_x: center_x,
                pos_y: center_y,
                collited: Collited::WithSnake,
                color: consts::RED,
            },
            body: vec![
                Block {
                    pos_x: center_x + 1,
                    pos_y: center_y,
                    collited: Collited::WithSnake,
                    color: consts::WHITE,
                },
                Block {
                    pos_x: center_x + 2,
                    pos_y: center_y,
                    collited: Collited::WithSnake,
                    color: consts::WHITE,
                },
                Block {
                    pos_x: center_x + 3,
                    pos_y: center_y,
                    collited: Collited::WithSnake,
                    color: consts::WHITE,
                },
                Block {
                    pos_x: center_x + 4,
                    pos_y: center_y,
                    collited: Collited::WithSnake,
                    color: consts::WHITE,
                },
            ],
            velocity: 6.0,
            direction: Direction::Left,
            direction_lock: false,
            growth_flag: false,
        }
    }

    // 检测蛇是否与自己的身体相碰撞
    fn is_colliting_with_self(&self) -> bool {
        let mut ans = false;
        for block in self.body.iter() {
            match self.head.is_collited_by_block(block) {
                Collited::NoCollision => continue,
                _ => {
                    ans = true;
                    break;
                }
            }
        }

        ans
    }

    // 蛇身加长的操作方法
    fn growth_action(&mut self) {
        // 设置 growth_flag
        self.growth_flag = true;
        // 增加移动速度
        self.velocity += 0.01;
    }

    // 移动
    fn moving(&mut self) {
        // 坐标移动
        let (x, y) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        // 克隆当前的坐标，会成为身体的一部分
        let mut pre_block = self.head.clone();
        pre_block.color = consts::WHITE;

        // 更新蛇头坐标
        self.head.pos_x += x;
        self.head.pos_y += y;

        // 通过将蛇体的当前块推到新向量来“移动”蛇
        let mut blocks = Vec::new();
        for block in self.body.iter_mut() {
            blocks.push(pre_block);
            pre_block = block.clone();
        }

        // 如果设置了增长标志，请不要浪费任何块。
        if self.growth_flag {
            blocks.push(pre_block);
            self.growth_flag = false;
        }

        // 分配新的身体
        self.body = blocks;
    }
}

/// 游戏状态机
#[derive(Debug, PartialEq, Clone)]
pub enum GameStatus {
    TIMEOUT,  // 游戏暂停
    GAMMING,  // 游戏进行中
    GAMEOVER, // 游戏结束
    RESTART,  // 游戏重启
}
