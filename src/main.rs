use macroquad::prelude::*;
use macroquad::audio::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_owned(),
        window_width: 288,
        window_height: 512,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw(texture: &Texture2D, x: f32, y: f32) {
    draw_texture(*texture, x, y, WHITE);
}

struct Bird{
    x: f32,
    y: f32,
    state: u8,
    angle: f32,
    speed: f32,
    accel: f32,
    height: f32,
    width: f32
}

fn draw_bird(bird: &mut Bird, textures: &Vec<Texture2D>){

    bird.y = bird.y + bird.speed;
    bird.speed = bird.speed + bird.accel;
    if bird.speed > 7.0{
        bird.speed = 7.0;
    }
    if bird.y + bird.height > 512.0-112.0{
        bird.y = 512.0-112.0 - bird.height
    }
    bird.angle = 0.3 + bird.speed/7.0;
    //draw(&textures[bird.state as usize], bird.x, bird.y)
    draw_texture_ex(*&textures[bird.state as usize], bird.x, bird.y, WHITE, DrawTextureParams{rotation: bird.angle, ..Default::default()})
}

struct Pipe{
    x: f32,
    y: f32,
    height: f32,
    width: f32,
    distance: f32
}

fn draw_pipe(pipe: &mut Pipe, textures: &Vec<Texture2D>) {
    if pipe.x < 0.0 - pipe.width{
        pipe.x = 228.0 + pipe.width;
        pipe.y = rand::gen_range(-238.0, -50.0);
    }
    draw(&textures[0], pipe.x, pipe.y);
    draw(&textures[1], pipe.x, pipe.y + pipe.distance + pipe.height);
}

fn is_colision(pipe: &Pipe, bird: &Bird) -> bool{
    if bird.y < 0.0 || bird.y ==  512.0-112.0 - bird.height || (bird.x + bird.width > pipe.x && bird.x < pipe.x + pipe.width) && (bird.y < pipe.y + pipe.height || bird.y > pipe.y + pipe.height + pipe.distance){
        return true;
    }else{
        return false;
    }
}

#[macroquad::main(window_conf())]
async fn main() {

    let mut move_pipe: bool = true;
    let mut take_input: bool = true;
    let mut stop_frame: u64 = 0;
    let mut check_colision: bool = true;
    let mut point_given: bool = false;

    let mut basex: f32 = 0.0;
    let mut frame_no: u64 = 0;

    let mut bird1 = Bird{x: 100.0, y: 144.0, state: 0, angle: 0.0, speed: 0.0, accel: 0.4, height: 24.0, width: 32.0};

    let mut pipe1 = Pipe{x: 288.0, y: -100.0, height: 320.0, width: 52.0, distance: 100.0};
    let mut pipe2 = Pipe{x: 288.0 + 144.0 + 26.0, y: -100.0, height: 320.0, width: 52.0, distance: 100.0};

    let wing_sound = load_sound("res/sfx_wing.wav").await.unwrap();
    let hit_sound = load_sound("res/sfx_hit.wav").await.unwrap();
    let die_sound = load_sound("res/sfx_die.wav").await.unwrap();
    let point_sound = load_sound("res/sfx_point.wav").await.unwrap();

    let texture_bg = Texture2D::from_file_with_format(
        include_bytes!("res/bg.png"),
        None
    );

    let texture_base = Texture2D::from_file_with_format(
        include_bytes!("res/base.png"),
        None
    );

    let texture_bird = vec![Texture2D::from_file_with_format(include_bytes!("res/bird1.png"),None),
                        Texture2D::from_file_with_format(include_bytes!("res/bird2.png"),None),
                        Texture2D::from_file_with_format(include_bytes!("res/bird3.png"),None)
                        ];

    let texture_pipe = vec![Texture2D::from_file_with_format(include_bytes!("res/pipe.png"),None),
                        Texture2D::from_file_with_format(include_bytes!("res/pipe2.png"),None)
                        ];


    loop {
        clear_background(BLACK);
        draw(&texture_bg, 0.0, 0.0);

        draw_pipe(&mut pipe1, &texture_pipe);
        draw_pipe(&mut pipe2, &texture_pipe);

        draw(&texture_base, basex, 512.0-112.0);
        draw_bird(&mut bird1, &texture_bird);

        if move_pipe{
            pipe1.x = pipe1.x - 3.0;
            pipe2.x = pipe2.x - 3.0;
            basex = basex - 3.0;
            if frame_no%7 == 0 {
                bird1.state = bird1.state + 1;
            }
        }

        if is_key_pressed(KeyCode::Up) && take_input {// || bird1.y + bird1.height > pipe1.y + pipe1.height + pipe1.distance{
            play_sound_once(wing_sound);
            bird1.speed = -7.0;
        }


        if basex < -42.0{
            basex = 0.0;
        }
        if bird1.state > 2{
            bird1.state = 0;
        }

        if point_given && frame_no != 0 && (frame_no)%((288/3 + 26 + 52)/3)==0{
            play_sound_once(point_sound);
        }

        if check_colision && (is_colision(&pipe1, &bird1) || is_colision(&pipe2, &bird1)){
            play_sound_once(hit_sound);
            play_sound_once(die_sound);
            draw_circle(10.0, 10.0, 30.0, YELLOW);
            move_pipe = false;
            take_input = false;
            stop_frame = frame_no + 100;
            check_colision = false;
            point_given = false;
        }


        if frame_no == stop_frame {
            frame_no = 0;
            stop_frame = 0;
            move_pipe = true;
            take_input = true;
            pipe1.x = 288.0;
            pipe2.x = 288.0 + 144.0 + 26.0;
            bird1.speed = 0.0;
            bird1.y = 144.0;
            check_colision = true;
            point_given = true;
        }

        frame_no = frame_no + 1;

        next_frame().await
    }
}