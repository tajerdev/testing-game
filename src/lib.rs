turbo::cfg! {r#"
    name = "Asteroid Shooter"
    version = "1.0.0"
    author = "Your Name"
    description = "Shoot down falling asteroids!"
    [settings]
    resolution = [256, 144]
    [solana]
   http-rpc-url = "http://localhost:8899"
   ws-rpc-url = "ws://localhost:8900"
"#}

use turbo::solana::solana_sdk::pubkey;
use turbo::solana::{anchor, solana_sdk};
use turbo::solana;
use solana_sdk::pubkey::Pubkey;
use anchor::Program;
use std::str::FromStr;
use solana_sdk::{instruction::AccountMeta};
use tryings::tryings::update_score;

turbo::init! {
    struct GameState {
        frame: u32,
        spaceship_x: f32,
        bullets: Vec<struct Bullet {
            x: f32,
            y: f32,
            vel: f32,
            size: f32,
        }>,
        asteroids: Vec<struct Asteroid {
            x: f32,
            y: f32,
            vel: f32,
            size: f32,
            hit: bool,
        }>,
        score: u32,
        can_shoot: bool,
        last_shoot_at: u32,
    
    

    } = { 
      
        Self {
            frame: 0,
            spaceship_x: 128.0,
            bullets: vec![],
            asteroids: vec![],
            score: 0,
            can_shoot: true,
            last_shoot_at: 0,
    
        }
    }
  

}

turbo::go! {
    let mut state = GameState::load();
  
    let user_pubkey = solana::user_pubkey();
   
    let program_id_result = Pubkey::from_str("8568N6s6P7BvVunKtM6ervUfysPSareZaujv4QTVjmGi");
    let program_id = program_id_result.unwrap_or_else(|err| {
        panic!("Program ID not parsing : {}", err);
    });
    
    let (pda_pubkey, bump_seed) = Pubkey::find_program_address(
        &[b"score"],
        &program_id,
    );

    
    if gamepad(0).left.pressed() {
        state.spaceship_x -= 2.0;
     
        state.can_shoot = false;
    }
    if gamepad(0).right.pressed() {
        state.spaceship_x += 2.0;
       
        state.can_shoot = false;
    }


   
    if gamepad(0).left.released() || gamepad(0).right.released() {
        state.can_shoot = true;
    }

   
    if !gamepad(0).left.pressed() && !gamepad(0).right.pressed() && gamepad(0).up.pressed() && state.can_shoot {
     
        let bullet = Bullet {
            x: state.spaceship_x,
            y: 120.0,
            vel: -5.0,
            size: 5.0,
        };
        state.bullets.push(bullet);


        state.can_shoot = false;

     
        state.last_shoot_at = state.frame;
    }

  
    if rand() % 64 == 0 {
        let asteroid = Asteroid {
            x: (rand() % 256) as f32,
            y: 0.0,
            vel: (rand() % 3 + 1) as f32,
            size: (rand() % 10 + 5) as f32,
            hit: false,
        };
        state.asteroids.push(asteroid);
    }

    
    for asteroid in &mut state.asteroids {
        for bullet in &mut state.bullets {
            let dx = bullet.x - asteroid.x;
            let dy = bullet.y - asteroid.y;
            let distance = (dx * dx + dy * dy).sqrt();
            let radii_sum = bullet.size + asteroid.size;

            if distance <= radii_sum && !asteroid.hit {
               
                state.score += 1;
                asteroid.hit = true;
            
            }
        }
             
    }
/* 
let instruction = tryings::UpdateScore {
    score: state.score, 
};
let accounts = vec![AccountMeta::new(user_pubkey, false), AccountMeta::new(pda_pubkey, false)]; 
let args = (instruction.score,); 
let tx = Program::new(program_id)
    .instruction("update_score")
    .accounts(accounts)
    .args(args)
    .send();
     
     
  
    // text!(&format!("User's: {:?}", tx));

         
            }
        }
    } */


    state.bullets.retain(|bullet| !state.asteroids.iter().any(|asteroid| {
        let dx = bullet.x - asteroid.x;
        let dy = bullet.y - asteroid.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let radii_sum = bullet.size + asteroid.size;
        distance <= radii_sum
    }));

  
    state.asteroids.retain(|asteroid| !asteroid.hit);

 
    state.asteroids.retain_mut(|asteroid| {
        asteroid.y += asteroid.vel;
    
        asteroid.y < 144. + (asteroid.size * 2.)
    });

    state.bullets.retain_mut(|bullet| {
        bullet.y += bullet.vel;
        bullet.y > 0. 
    });

     // Set the background color
     clear(0x00ffffff);


     let frame = (state.frame as i32) / 2;
     for col in 0..9 {
         for row in 0..6 {
             let x = col * 32;
             let y = row * 32;
             let x = ((x + frame) % (272 + 16)) - 32;
             let y = ((y + frame) % (144 + 16)) - 24;
             sprite!("heart", x = x, y = y);
         }
     }
 


    sprite!("jetsss", x = state.spaceship_x as i32, y = 120);


 if state.frame >= 64 && state.frame.saturating_sub(state.last_shoot_at) <= 60 {
    rect!(w = 30, h = 10, x = state.spaceship_x as i32 + 32, y = 110);
    circ!(d = 10, x = state.spaceship_x as i32 + 28, y = 115);
    rect!(w = 10, h = 5, x = state.spaceship_x as i32 + 28, y = 110);
    circ!(d = 10, x = state.spaceship_x as i32 + 56, y = 115);
    text!("Pew Pew!", x = state.spaceship_x as i32 + 33, y = 113, font = Font::S, color = 0x000000ff);
} 


    
    for asteroid in &state.asteroids {
        sprite!("taco", x = asteroid.x as i32, y = asteroid.y as i32, fps = fps::FAST); // Render the asteroids
    }
    for bullet in &state.bullets {
        circ!(x = bullet.x as i32, y = bullet.y as i32, d = bullet.size as u32, fill = 0x00ff00ff); // Render the bullets
    }

    text!(&format!("Score: {}", state.score), x = 10, y = 10, font = Font::L, color = 0xffffffff); // Render the score
   
     text!(
        &format!("User's Public Key: {:?}", user_pubkey),
        x = 10,
        y = 30,
        font = Font::S,
        color = 0xffffffff
    );  
   

    state.frame += 1;
    state.save();
    }


    

