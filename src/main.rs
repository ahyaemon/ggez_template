extern crate cgmath;
extern crate ggez;

use ggez::{
    Context,
    GameResult,
    graphics,
    audio::{
        self,
        SoundSource,
    },
    event::{
        self,
        KeyCode,
        KeyMods,
    },
};

use std::{
    env,
    path,
};

struct MainState {
    // プレイヤーの画像
    image: graphics::Image,

    // プレイヤーの情報
    x: f32,
    y: f32,
    speed: f32,

    // bgm, se
    bgm: audio::Source,
    se: audio::Source,

    // キー入力からの操作系
    up_pressing: bool,
    right_pressing: bool,
    down_pressing: bool,
    left_pressing: bool,
    space_pressed: bool,
}

impl MainState {

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "/stand01.png")?;
        let bgm = audio::Source::new(ctx, "/juvenile.mp3")?;
        let se = audio::Source::new(ctx, "/hyoushigi1.mp3")?;

        Ok(
            MainState{
                image,

                x: 100.0,
                y: 100.0,
                speed: 8.0,

                bgm,
                se,

                up_pressing: false,
                right_pressing: false,
                down_pressing: false,
                left_pressing: false,
                space_pressed: false,
            }
        )
    }

}

impl event::EventHandler for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // 移動制御
        if self.up_pressing { self.y -= self.speed; }
        if self.right_pressing { self.x += self.speed; }
        if self.down_pressing { self.y += self.speed; }
        if self.left_pressing { self.x -= self.speed; }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        // プレイヤーの描画
        let dest = cgmath::Point2::new(self.x, self.y);
        graphics::draw(ctx, &self.image, (dest,))?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::A => { self.left_pressing = true; }
            KeyCode::S => { self.down_pressing = true; }
            KeyCode::D => { self.right_pressing = true; }
            KeyCode::W => { self.up_pressing = true; }
            // 最初の key_down の時のみ効果音を鳴らす
            KeyCode::Space => {
                if self.space_pressed == false {
                    let _  = self.se.play();
                }
                self.space_pressed = true
            }
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::A => { self.left_pressing = false; }
            KeyCode::S => { self.down_pressing = false; }
            KeyCode::D => { self.right_pressing = false; }
            KeyCode::W => { self.up_pressing = false; }
            KeyCode::Space => { self.space_pressed = false; }
            _ => {}
        }
    }

}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);
    let (ctx, events_loop) = &mut cb.build()?;
    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx).unwrap();

    // bgm をリピートで流す
    &state.bgm.set_repeat(true);
    &state.bgm.play_detached();

    // ゲーム開始
    event::run(ctx, events_loop, state)
}
