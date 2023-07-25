//reference https://tetra.seventeencups.net/tutorial
//resources  www.kenney.nl

use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_W: f32 = 640.0;
const WINDOW_H: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0; 
const BALL_SPEED: f32 = 3.5; 
const PADDLE_SPIN: f32 = 4.0; 
const BALL_ACC: f32 = 0.05;

struct Entity { 
    texture: Texture, 
    position: Vec2<f32>,
    velocity: Vec2<f32>, 
}

impl Entity {
    //constructor
    fn new(texture: Texture, position: Vec2<f32> ) -> Entity{ 
        Entity::width_velocity( texture, position, Vec2::zero() )
    }

    fn width_velocity( texture: Texture, position: Vec2<f32>, velocity: Vec2<f32> ) -> Entity{ 
        Entity { texture, position, velocity }
    }

    fn width(&self) ->f32 { 
        self.texture.width() as f32 
    }

    fn height(&self) -> f32 { 
        self.texture.height() as f32 
    }

    fn bounds(&self)-> Rectangle { 
        Rectangle::new(
            self.position.x, 
            self.position.y, 
            self.width(), 
            self.height(), 
        )
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new (
            self.position.x + (self.width() / 2.0), 
            self.position.y + (self.height() / 2.0),
        )
    }
}

struct Enemy { 
    body: Entity, 
    health: u32,
    rocket1: Entity,
    rocket2: Entity,
}

struct Boss { 
    obj: Enemy, 
}

enum WeaponType{ 
    ROCKETS,
    WEAPONS_CNT,
    FATROCKET, 
    SHUTGUN,  
    MINIGUN, 
    LAZER,
    NUKE,
    BLACKHOLE,
}

struct Weapon { 
    attack: u32, 
    health: u32, 
    fireRate: u32, 
    fireRateTimer u32,
    rounds: u32,
    round_cnt: u32,
    reloadSpeed: u32,
}

impl Weapon { 

    fn new(  attack: u32, health: u32, fireRate: u32, 
        fireRateTimer u32, clip_size: u32, rounds: u32 )-> Self 
    {       

        Weapon{ 
            attack: attack, 
            health: health, 
            fireRate: fireRate, 
            fireRateTimer fireRateTimer,
            clip_size: clip_size,
            rounds: rounds,
            reloadSpeed: reloadSpeed,
        }

    }

}

struct Rocket { 
    body: Entity,
    attack: u32,
    health: u32,
}

impl Rocket{ 

    fn new(ctx: &mut Context, posx: f32, posy: f32) -> Self { 

        let mut ball_texture = Texture::new(ctx,"../resources/puzzlepack/png/ballGrey.png").unwrap();
        let ball_position = Vec2::new( 
            posx, 
            posy, 
        );
    
        let ball_velocity = Vec2::new(BALL_SPEED, 0.0);
        let bodye = Entity::width_velocity(ball_texture, ball_position, ball_velocity);
        
        Rocket{ 
            body: bodye,
            attack: 5,
            health: 5,
        }
    }

    //fn draw(self, ctx:&mut Context) ->() {


    //}

    // fn update(&mut self){ 
        
    //     for rocket in &mut self.rocketsLeft{ 
    //         rocket.body.position += rocket.body.velocity
    //     }

    //     for rocket in &mut self.rocketsRight{ 
    //         rocket.body.position += rocket.body.velocity
    //     }

    // }
}

struct Rockets{
    parent: Weapon,
    rocketsLeft: Vec<Rocket>,
    rocketsRight: Vec<Rocket>,
}

impl Rockets { 

    fn new()-> Self { 

        let parent = Weapon::new(5, 5, 30, 30, 30, 10, 15);
        let rounds = 10;
        Rockets{ 
            parent: 
            rocketsLeft: vec![-1; rounds],
            rocketsRight: vec![-1; rounds],
        }
    }

    fn draw(self, ctx:&mut Context) -> () {
        for rocket in &mut self.player1.rocketsLeft{ 
            rocket.body.texture.draw(ctx, rocket.body.position);
        }

        for rocket in &mut self.player1.rocketsRight{ 
            rocket.body.texture.draw(ctx, rocket.body.position);
        }

        // use itertools::Itertool; 
        // use itertools::EitherOrBoth::{Both, Left, RIght};

        // for itr in self.player1.rocketsLeft.iter().zip_longest(self.player1.rocketsRight.iter()){ 
        //     match itr{ 
        //         Both(x,y) => x.texture.draw(ctx, x.position);  y.texture.draw(ctx, y.position), 
        //         Left(x) => x.texture.draw(ctx, x.position), 
        //         Right(y) => y.texture.draw(ctx, y.position),
        //     }
        // }
    }

    fn fire(&mut self, ctx:&mut Context) -> () { 
        if self.fireRateTimer >= self.fireRate{ 
            self.rocketsLeft.push(Rocket::new(ctx, self.body.position.x, self.body.position.y));
            self.rocketsRight.push(Rocket::new(ctx, self.body.position.x, self.body.position.y + self.body.height()-25.0));
            self.fireRateTimer = 0;
        }
    }
}

struct Player{ 
    body: Entity, 
    score: u32,
    health: u32, 
    name: String,
    rocketsLeft: Vec<Rocket>,
    rocketsRight: Vec<Rocket>, 
    fireRate: u32, 
    fireRateTimer: u32,
    current_weapon: WeaponType,
}

impl Player{ 
    fn new(ctx: &mut Context, name: String) -> tetra::Result<Self>{
        //TODO handle error case. 
        let player_texture = Texture::new(ctx, "../resources/puzzlepack/png/paddleBlu.png")?;
        let player_position = Vec2::new(16.0, (WINDOW_H - player_texture.height() as f32) /2.0);
        let ob = Entity::new(player_texture, player_position);
        let weapons = vec[Rockets::new; WeaponType::WEAPONS_CNT ]
        Ok(
        Player{ 
            body: ob,
            score: 0,
            health: 100, 
            name: name, 
            rocketsLeft: vec![], 
            rocketsRight: vec![],
            fireRate: 30, 
            fireRateTimer: 30,
            current_weapon: WeaponType::ROCKET,
        }
        )
    }

    // fn change_weapon() -> () { 
    // }

    fn draw(self, ctx:&mut Context) -> () {
        
        self.body.texture.draw(ctx, self.body.position); 

        weapons[current_weapon].draw(crx);

    }

    fn fire(&mut self, ctx:&mut Context) -> () { 

        // match current_weapon { 
        //   current_weapon::Rocket => Weapon.fire(),
        //   current_weapon::FATROCKET =>   ,
        //   current_weapon::SHUTGUN =>   ,
        //   current_weapon::MINIGUN =>   ,
        //   current_weapon::LAZER =>   ,
        //   current_weapon::NUKE =>   ,
        //   current_weapon::BLACKHOLE =>   ,
        //   current_weapon::BLACKHOLE =>   ,
        // }
        weapons[current_weapon].fire(crx);
    }

    fn update(&mut self) ->(){ 
        self.updateFireTimer();

        for rocket in &mut self.rocketsLeft{ 
            rocket.body.position += rocket.body.velocity
        }

        for rocket in &mut self.rocketsRight{ 
            rocket.body.position += rocket.body.velocity
        }

        // for itr in player1.rocketsLeft.iter().zip_longest(rocketsRight.iter()){ 
        //     match itr{ 
        //         Both(x,y) => x.texture.draw(ctx, x.position);  y.texture.draw(ctx, y.position), 
        //         Left(x) => x.texture.draw(ctx, x.position), 
        //         Right(y) => y.texture.draw(ctx, y.position),
        //     }
        // }
    }

    fn updateFireTimer(&mut self) ->() { 
        if self.fireRateTimer < self.fireRate {
            self.fireRateTimer +=1
        }
    }
}
/* 
struct level{

}
*/ 

struct GameState {
    player1: Player, 
    //blocks: Vec<Object>, 
    //enemies: Vec<Enemy>,
    //score_label: Entity, 
}

impl State for GameState{
    // call time varies
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result { 
        graphics::clear(ctx, Color::rgb(0.999, 0.4, 0.1));

        self.player1.draw(ctx);

        // for rocket in &mut self.player1.rocketsLeft{ 
        //     rocket.body.texture.draw(ctx, rocket.body.position);
        // }

        // for rocket in &mut self.player1.rocketsRight{ 
        //     rocket.body.texture.draw(ctx, rocket.body.position);
        // }

        Ok(())
    }

    // call time 60 hrz
    fn update(&mut self, ctx: &mut Context) -> tetra::Result { 
        
        if input::is_key_down(ctx, Key::Up) { 
            if self.player1.body.position.y >= 0.0 {
                self.player1.body.position.y -= PADDLE_SPEED;
            }
        }

        if input::is_key_down(ctx, Key::Down) { 
            if self.player1.body.position.y <= WINDOW_H - self.player1.body.texture.height() as f32 {
                self.player1.body.position.y += PADDLE_SPEED;
            }
        }

        if input::is_key_down(ctx, Key::Space) { 
            self.player1.fire(ctx); 
        }

        self.player1.update(); 

        // let player1_bounds = self.player1.bounds(); 
        // let ball_bounds = self.ball.bounds(); 

        // let paddle_hit = if ball_bounds.intersects(&player1_bounds){ 
        //     Some(&self.player1)
        // } else { 
        //     None
        // };
        
        // if let Some(paddle) = paddle_hit { 
        //     self.player1.heath -= 1; 
        // }
        
        // if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_H {
        //     self.ball.velocity.y = -self.ball.velocity.y; 
        // } 
       
        Ok(())
    }
} 

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> { 
        //TODO handle error case. 
        let player = Player::new( ctx,"Zee".to_string()).unwrap();
        
        Ok(GameState {
            player1: player, 
        })
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Space Warz", WINDOW_W as i32, WINDOW_H as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new) 
}
