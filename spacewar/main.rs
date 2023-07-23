//reference https://tetra.seventeencups.net/tutorial
//resources  www.kenney.nl

use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_W: f32 = 640.0;
const WINDOW_H: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0; 
const BALL_SPEED: f32 = 5.0; 
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

struct Object { 
    body: Entity,
    health: u32,
}

impl Object{ 

    fn new(body: Entity, health: u32) -> Self { 
        Object{ 
            body, 
            health,
        }
    }
}

struct Enemy { 
    obj: Object, 
    rocket1: Entity,
    rocket2: Entity,
}

struct Boss { 
    obj: Enemy, 
}

struct Rocket { 
    obj: Object,
    attack: u32,
}

impl Rocket{ 

    fn new(ctx: &mut Context, posx: u32, posy: u32) -> Self { 

        let mut ball_texture = Texture::new(ctx,"./resources/puzzlepack/png/ballGrey.png")?;
        let ball_position = Vec2::new( 
            posx, 
            posy, 
        );
    
        let ball_velocity = Vec2::new(BALL_SPEED, 0.0);
        let body = Entity::width_velocity(ball_texture, ball_position, ball_velocity);
        let obj_param = Object::new(body, 5);
        
        Rocket{ 
            obj: obj_param,
            attack: 5,
        }
    }
}

struct Player{ 
    obj: Object, 
    score: u32, 
    name: String,
    rocketsLeft: Vec<Rocket>,
    rocketRight: Vec<Rocket>, 
}

impl Player{ 
    fn new(ctx: &mut Context, name: String) -> self{ 
        let player_texture = Texture::new(ctx, "./resources/puzzlepack/png/paddleBlu.png")?;
        let player_position = Vec2::new(16.0, (WINDOW_H - player_texture.height() as f32) /2.0);
        let ob = Object::new(Entity::new(player_texture, player_position), 100);
        
        Player{ 
            obj: ob,
            score: 0, 
            name: name, 
            rocketsLeft: vec![], 
            rocketRight: vec![],
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
        
        self.player1.texture.draw(ctx, self.player1.position);

        use itertools::Itertool; 
        use itertools::EitherOrBoth::{Both, Left, RIght};

        // for itr in self.player1.rocketsLeft.iter().zip_longest(self.player1.rocketsRight.iter()){ 
        //     match itr{ 
        //         Both(x,y) => x.texture.draw(ctx, x.position);  y.texture.draw(ctx, y.position), 
        //         Left(x) => x.texture.draw(ctx, x.position), 
        //         Right(y) => y.texture.draw(ctx, y.position),
        //     }
        // }

        Ok(())
    }

    // call time 60 hrz
    fn update(&mut self, ctx: &mut Context) -> tetra::Result { 
           //Up Down 
        if input::is_key_down(ctx, Key::Up) { 
            if self.player1.position.y >= 0.0 {
                self.player1.position.y -= PADDLE_SPEED;
            }
        }

        if input::is_key_down(ctx, Key::Down) { 
            if self.player1.position.y <= WINDOW_H - self.player1.texture.height() as f32 {
                self.player1.position.y += PADDLE_SPEED;
            }
        }

        if input::is_key_down(ctx, Key::Space) { 
            self.player1.rocketsLeft.push_back(Rocket::new(ctx, self.player1.position.y + 10, self.player1.position.y));
            self.player1.rocketsRight.push_back(Rocket::new(ctx, self.player1.position.y, self.player1.position.y ));
        }

        
        // self.ball.position += self.ball.velocity;

        // let player1_bounds = self.player1.bounds(); 
        // let ball_bounds = self.ball.bounds(); 

        // let paddle_hit = if ball_bounds.intersects(&player1_bounds){ 
        //     Some(&self.player1)
        // } else { 
        //     None
        // };
        
        // if let Some(paddle) = paddle_hit { 
        //     self.player1.obj.heath -= 1; 
        // }
        
        
        // for itr in player1.rocketsLeft.iter().zip_longest(rocketsRight.iter()){ 
        //     match itr{ 
        //         Both(x,y) => x.texture.draw(ctx, x.position);  y.texture.draw(ctx, y.position), 
        //         Left(x) => x.texture.draw(ctx, x.position), 
        //         Right(y) => y.texture.draw(ctx, y.position),
        //     }
        // }

        // if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_H {
        //     self.ball.velocity.y = -self.ball.velocity.y; 
        // } 
       
        Ok(())
    }
} 

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> { 
        
        Ok(GameState {
            player1: Player::new("Zee"), 
        })
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Space Warz", WINDOW_W as i32, WINDOW_H as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new) 
}
