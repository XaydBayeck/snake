use crate::{Block, Fruit, Snake, Wall};
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub trait Render {
    fn render(&self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context);

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
    fn render(&self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context) {
        rectangle(
            self.color,
            Block::renderable_rect(circus, &self, args),
            c.transform,
            gl,
        );
    }
}

impl Render for Wall {
    fn render(&self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context) {
        for block in self.bricks.iter() {
            block.render(circus, args, gl, c);
        }
    }
}

impl Render for Fruit {
    fn render(&self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context) {
        self.block.render(circus, args, gl, c);
    }
}

impl Render for Snake {
    fn render(&self, circus: &[u32; 2], args: &RenderArgs, gl: &mut GlGraphics, c: Context) {
        self.head.render(circus, args, gl, c);

        for block in self.body.iter() {
            block.render(circus, args, gl, c);
        }
    }
}
