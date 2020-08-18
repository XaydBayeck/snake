use collision::Collited;
use rand::Rng;
use std::cmp::min;

/// 应用程序主体
mod app;
/// 碰撞系统
mod collision;
/// 包含会用到的预设常量
mod consts;

fn main() {}

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
        let width = circus[0] as i32 + 2;
        let height = circus[1] as i32;

        let wallColor = consts::LIGHTBLUE;
        let mut bricks = Vec::<Block>::new();

        // 添加上下俩个边框
        for i in 0..width {
            let brickUp = Block::new(i, height, Collited::WithWall, wallColor);
            let brickDown = Block::new(i, 0, Collited::WithWall, wallColor);

            bricks.push(brickUp);
            bricks.push(brickDown);
        }

        // 添加左右俩个边框
        for i in 0..height {
            let brickLeft = Block::new(i, 0, Collited::WithWall, wallColor);
            let brickRight = Block::new(i, width, Collited::WithWall, wallColor);

            bricks.push(brickLeft);
            bricks.push(brickRight);
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
            let (dx, dy) = match rand::thread_rng().gen_bool(0.5) {
                true => (rand::thread_rng().gen_range(-1, 1), 0),
                false => (0, rand::thread_rng().gen_range(-1, 1)),
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
                x_pre + dx,
                y_pre + dy,
                Collited::WithWall,
                consts::LIGHTBLUE,
            );
            bricks.push(brick);
        }

        Wall { bricks }
    }
}

/// 食物实体
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
}

impl Snake {
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
        }
    }
}
