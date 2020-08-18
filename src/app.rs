use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{RenderArgs, UpdateArgs};

/// 蛇的移动方向
#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// 应用程序主体结构体
#[derive(Clone)]
struct App {}

impl App {
    /// 建立新的App实例
    pub fn new(horizontal_block_num: u32, vertical_block_num: u32) -> Self {}

    /// 理论计算更新主函数
    fn update(&mut self, args: &UpdateArgs) {}

    /// 渲染方法的主函数
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, glyp_cache: &mut GlyphCache) {}

    /// 运行程序
    fn run(&self) {}
}
