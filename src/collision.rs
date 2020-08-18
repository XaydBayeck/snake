use super::{Block, Fruit, Snake, Wall};

#[derive(Debug, Clone, PartialEq)]
pub enum Collited {
    WithFruit,
    WithSnake,
    WithWall,
    NoCollision,
}

trait Collision {
    /// 检测实体是否与另一个可碰撞实体碰撞
    /// 返回值为另一个实体的碰撞检测类型
    fn is_colliting(&mut self, object: impl Collision) -> Collited;

    /// 检测实体是否被一个Block实体碰撞
    /// 返回值为该实体的碰撞检测类型
    fn is_collited_by_block(&mut self, block: &Block) -> Collited;
}

impl Collision for Block {
    /// 检测Block实体与其他类型的实体是否碰撞
    fn is_colliting(&mut self, block: impl Collision) -> Collited {
        block.is_collited_by_block(self)
    }

    /// 检测Block实体是否与另外一个Block实体碰撞
    fn is_collited_by_block(&mut self, block: &Block) -> Collited {
        if self.pos_x == block.pos_x && self.pos_y == block.pos_y {
            return self.collited;
        }

        Collited::NoCollision
    }
}

impl Collision for Wall {
    fn is_colliting(&mut self, object: impl Collision) -> Collited {
        let mut ans = Collited::NoCollision;
        for brick in self.bricks.iter() {
            match object.is_collited_by_block(brick) {
                Collited::NoCollision => continue,
                other => {
                    ans = other;
                    break;
                }
            }
        }

        ans
    }

    fn is_collited_by_block(&mut self, block: &Block) -> Collited {
        let mut ans = Collited::NoCollision;
        for brick in self.bricks.iter() {
            match brick.is_collited_by_block(block) {
                Collited::NoCollision => continue,
                _ => {
                    ans = Collited::WithWall;
                    break;
                }
            }
        }

        ans
    }
}

impl Collision for Fruit {
    fn is_colliting(&mut self, object: impl Collision) -> Collited {
        object.is_collited_by_block(&self.block)
    }

    fn is_collited_by_block(&mut self, block: &Block) -> Collited {
        self.block.is_collited_by_block(block)
    }
}

impl Collision for Snake {
    fn is_colliting(&mut self, object: impl Collision) -> Collited {
        match self.head.is_colliting(object) {
            Collited::NoCollision => {
                let ans = Collited::NoCollision;
                for block in self.body.iter() {
                    match object.is_collited_by_block(block) {
                        Collited::NoCollision => continue,
                        other => {
                            ans = other;
                            break;
                        }
                    }
                }

                ans
            }
            other => other,
        }
    }

    fn is_collited_by_block(&mut self, block: &Block) -> Collited {
        match self.head.is_collited_by_block(block) {
            Collited::NoCollision => {
                let ans = Collited::NoCollision;
                for block in self.body.iter() {
                    match block.is_collited_by_block(block) {
                        Collited::NoCollision => continue,
                        other => {
                            ans = Collited::WithSnake;
                            break;
                        }
                    }
                }

                ans
            }
            _ => Collited::WithSnake,
        }
    }
}

impl Snake {
    fn is_colliting_with_self(&self) -> bool {
        let ans = false;
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
}
