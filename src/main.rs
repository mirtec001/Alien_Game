use macroquad::prelude::*;

extern crate rand;
use crate::rand::Rng;

struct Player {
    pos: Vec2,
    velocity: Vec2,
    speed: f32,
    shot_timer: f64,
}

struct Bullet {
    pos: Vec2,
    speed: f32,
    collided: bool,
}

struct Alien {
    pos: Vec2,
    speed: f32,
    shot_timer: f32,
    collided: bool,
}

fn generate_aliens(cell: f32, speed: f32) -> Vec<Alien> {
    let mut rng = rand::thread_rng();
    let mut aliens = Vec::new();
    for _ in 0..32 {
        aliens.push(Alien {
            pos: Vec2::new(rng.gen_range(0.0..screen_width() - cell), rng.gen_range(-100.0..-cell)),
            speed: rng.gen_range(0.5..speed),
            shot_timer: rng.gen_range(0.0..2.5),
            collided: false,
        });
    }
    return aliens;
}


#[macroquad::main("Alien")]
async fn main() {
    let cell = 32.;
    let mut last_shot = get_time();
    let padding = 40.;
    let mut bullets = Vec::new();
    let mut aliens = Vec::new();
    let mut level = 1;

    let mut score = 0;

    let mut player = Player {
        pos: Vec2::new(100., screen_height() - (padding + cell)),
        velocity: Vec2::new(0., 0.),
        speed: 5.,
        shot_timer: 0.5,
    };

    aliens = generate_aliens(cell, 4.);

    loop {

        clear_background(BLACK);
        player.velocity = Vec2::new(0., 0.);
        let current_time = get_time();
        let text = format!("Score: {}", score);

        // Input
        if is_key_down(KeyCode::Left) {
            player.velocity.x -= player.speed;
        }

        if is_key_down(KeyCode::Right) {
            player.velocity.x += player.speed;
        }
        // println!("{}", (current_time - last_shot));
        if is_key_down(KeyCode::Space) && (current_time - last_shot) > player.shot_timer {
            bullets.push(Bullet {
                pos: player.pos,
                speed: 5.,
                collided: false,

            });
            last_shot = current_time;
        }
        
        // Update

        if aliens.len() > 0 {
            player.pos += player.velocity;
            for alien in aliens.iter_mut() {
                alien.pos.y += alien.speed;
                
                for bullet in bullets.iter_mut() {
                    if (alien.pos - bullet.pos).length() < cell {
                        score += 1;
                        alien.collided = true;
                        bullet.collided = true;
                    } else {
                        bullet.pos.y -= bullet.speed;
                    }
                }
                break;
            }
            
            bullets.retain(|bullet| !bullet.collided);
            aliens.retain(|alien| !alien.collided);
            aliens.retain(|alien| alien.pos.y < (screen_height() - cell));
        }
        else {
            level = level + 1;
            bullets = Vec::new();
            aliens = generate_aliens(cell, (level as f32 + 4.));
        }
        
        
        
        // Draw
        draw_rectangle(player.pos.x, player.pos.y, cell, cell, RED);
        for bullet in bullets.iter_mut() {
            draw_rectangle(bullet.pos.x, bullet.pos.y, 4., 8., YELLOW);
        }
        for alien in aliens.iter_mut() {
            draw_rectangle(alien.pos.x, alien.pos.y, cell, cell, GREEN);
        }

        draw_text(&text, 20., 20., 14., WHITE);
        next_frame().await;
    }

}