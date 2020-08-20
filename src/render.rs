use crate::{app::App, consts, Block, Fruit, GameStatus, Snake, Wall};
use graphics::*;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::RenderArgs;

pub trait Render {
    fn render(&mut self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context);

    fn renderable_rect(circus: &[u32; 2], block: &Block, args: &RenderArgs) -> [f64; 4] {
        let block_size_x = args.window_size[0] / (circus[0] as f64);
        let block_size_y = args.window_size[1] / (circus[1] as f64);
        let window_pos_x = (block.pos_x as f64) * block_size_x;
        let window_pos_y = (block.pos_y as f64) * block_size_y;

        rectangle::rectangle_by_corners(
            window_pos_x - block_size_x * 0.5,
            window_pos_y - block_size_y * 0.5,
            window_pos_x + block_size_x * 0.5,
            window_pos_y + block_size_y * 0.5,
        )
    }
}

impl Render for Block {
    fn render(&mut self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context) {
        rectangle(
            self.color,
            Block::renderable_rect(circus, &self, args),
            c.transform,
            gl,
        );
    }
}

impl Render for Wall {
    fn render(&mut self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context) {
        for block in self.bricks.iter_mut() {
            block.render(circus, args, gl, c);
        }
    }
}

impl Render for Fruit {
    fn render(&mut self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context) {
        self.block.render(circus, args, gl, c);
    }
}

impl Render for Snake {
    fn render(&mut self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context) {
        self.head.render(circus, args, gl, c);

        for block in self.body.iter_mut() {
            block.render(circus, args, gl, c);
        }
    }
}

impl Render for App<'_> {
    fn render(&mut self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context) {

        // 清空屏幕
        clear(consts::BLACK, gl);

        // 绘制分数
        text(
            color::WHITE,
            15,
            format!("Your score is {}", self.score).as_str(),
            &mut self.glyph_cache,
            c.transform.trans(10.0, 20.0),
            gl,
        )
            .unwrap();

        match self.game_status {
            GameStatus::TIMEOUT => {
                // 绘制蛇
                self.snake.render(&self.circus, args, gl, c);

                // 绘制边框
                self.board_wall.render(&self.circus, args, gl, c);

                // 绘制墙壁
                for wall in self.walls.iter_mut() {
                    wall.render(circus, args, gl, c);
                }

                // 绘制食物
                self.fruit.render(circus, args, gl, c);

                // 绘制暂停幕布
                let curtain = rectangle::rectangle_by_corners(
                0.0,
                0.0,
                args.window_size[0],
                args.window_size[1],
                );
                rectangle(consts::GRAY, curtain, c.transform, gl);

                // 绘制暂停信息
                text(
                    consts::ORANGE,
                    15,
                    format!("Game stop!Continue by press Enter,quit by press Esc.").as_str(),
                    &mut self.glyph_cache,
                    c.transform.trans(10.0, 40.0),
                    gl,
                )
                .unwrap();
            }
            GameStatus::GAMING => {
                // 绘制蛇
                self.snake.render(&self.circus, args, gl, c);

                // 绘制边框
                self.board_wall.render(&self.circus, args, gl, c);

                // 绘制墙壁
                for wall in self.walls.iter_mut() {
                    wall.render(&self.circus, args, gl, c);
                }

                // 绘制食物
                self.fruit.render(&self.circus, args, gl, c);
            }
            GameStatus::GAMEOVER => {
                // 显示游戏结束和分数
                text(
                    consts::RED,
                    15,
                    format!("Game over! Press Space to restart, Escape to quit!").as_str(),
                    &mut self.glyph_cache,
                    c.transform.trans(10.0, 40.0),
                    gl,
                )
                .unwrap();
            }
            GameStatus::RESTART => (),
        }
    }
}
