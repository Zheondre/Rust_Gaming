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

//constructor
impl Entity {
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

struct GameState {
    player1: Entity, 
    player2: Entity, 
    ball: Entity,
    //score_label: Entity, 
}

impl State for GameState{
    // call time varies
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result { 
        graphics::clear(ctx, Color::rgb(0.999, 0.4, 0.1));
        
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);

        Ok(())
    }

    // call time 60 hrz
    fn update(&mut self, ctx: &mut Context) -> tetra::Result { 
           //Up Down 
        if input::is_key_down(ctx, Key::W) { 
            if self.player1.position.y >= 0.0 {
                self.player1.position.y -= PADDLE_SPEED;
            }
        }

        if input::is_key_down(ctx, Key::S) { 
            if self.player1.position.y <= WINDOW_H - self.player1.texture.height() as f32 {
                self.player1.position.y += PADDLE_SPEED;
            }
        }

        if input::is_key_down(ctx, Key::Up) { 
            if self.player2.position.y >= 0.0 { 
                self.player2.position.y -= PADDLE_SPEED;
            }
        }

        if input::is_key_down(ctx, Key::Down) { 
            if self.player2.position.y <= WINDOW_H - self.player2.texture.height() as f32 {
                self.player2.position.y += PADDLE_SPEED;
            }
        }

        self.ball.position += self.ball.velocity;

        let player1_bounds = self.player1.bounds(); 
        let player2_bounds = self.player2.bounds(); 
        let ball_bounds = self.ball.bounds(); 

        let paddle_hit = if ball_bounds.intersects(&player1_bounds){ 
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds){
            Some(&self.player2)
        } else { 
            None
        };

        if let Some(paddle) = paddle_hit { 
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum())); 
            
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height(); 

            self.ball.velocity.y += PADDLE_SPIN * -offset; 
        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_H {
            self.ball.velocity.y = -self.ball.velocity.y; 
        }
       
        Ok(())
    }
} 

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> { 
        let player1_texture = Texture::new(ctx, "./resources/puzzlepack/png/paddleBlu.png")?;
        let player2_texture = Texture::new(ctx, "./resources/puzzlepack/png/paddleRed.png")?;
        let ball_texture = Texture::new(ctx,"./resources/puzzlepack/png/ballGrey.png")?;

        let player1_position = Vec2::new(16.0, (WINDOW_H - player1_texture.height() as f32) /2.0);
        let player2_position = Vec2::new(
            WINDOW_W - player2_texture.width() as f32 - 16.0,
            (WINDOW_H - player2_texture.height() as f32) /2.0
        );
        let ball_position = Vec2::new( 
            WINDOW_H / 2.0 - ball_texture.width() as f32 / 2.0, 
            WINDOW_H / 2.0 - ball_texture.width() as f32 / 2.0 
        ); 
        let ball_velocity = Vec2::new( -BALL_SPEED, 0.0);
        
        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position), 
            player2: Entity::new(player2_texture, player2_position), 
            ball: Entity::width_velocity(ball_texture, ball_position, ball_velocity),
        })
    }
}


fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_W as i32, WINDOW_H as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new) 
}
