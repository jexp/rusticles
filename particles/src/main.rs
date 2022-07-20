use speedy2d::color::Color;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{WindowHandler, WindowHelper, MouseButton};
// use core::f32::consts::PI;
use rand::Rng;
use speedy2d::dimen::Vector2;
use std::time::Instant;

fn main() {
    let window = Window::new_centered("Title", (1000, 1000)).unwrap();
    let center = Vector2::from((500.0,500.0));
    window.run_loop(MyWindowHandler{ angle : 0.0, alpha : 0.0, 
        particle:Particle::new(center),
        particles: Vec::new(),
        mouse: center,
        button_down_start: None
    });
}

struct MyWindowHandler { 
    angle : f32,
    alpha : f32,
    particle: Particle,
    particles: Vec<Particle>,
    mouse : Vector2<f32>,
    button_down_start : Option<Instant>
}

struct Particle {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    lifespan: f32,
    color: Color
}

impl Particle {
   fn new( position: Vector2<f32>) -> Self {
    let mut rng = rand::thread_rng();
    let green_blue = rng.gen::<f32>();
    Self {  position : position, 
            acceleration : Vector2::from((0.0, 0.03)), 
            velocity: Vector2::from((rng.gen_range(-3.0..=3.0), rng.gen_range(-1.0..=3.0))), 
            lifespan: 255.0,
            color: Color::from_rgb(rng.gen::<f32>(), 1.0, green_blue)
         }
   }

   fn run(&mut self, graphics: &mut Graphics2D) {
    self.update();
    self.display(graphics);
  }

  fn update(&mut self) {
    self.velocity = self.velocity + self.acceleration;
    self.position = self.position + self.velocity;
    self.lifespan -= 2.0;
  }

  fn display(&mut self, graphics: &mut Graphics2D) {
    let col = Color::from_rgba(self.color.r(), self.color.g(), self.color.b(), self.lifespan / 255.0);
    graphics.draw_circle(self.position, 8.0, col);
  }

  fn is_alive(&mut self) -> bool {
     self.lifespan > 0.0
  }
}

impl MyWindowHandler {
    fn add_particle(&mut self) {
        let cx : f32 = self.mouse.x; // 500.0
        let cy : f32 = self.mouse.y; // 500.0
        self.particles.push(Particle::new(Vector2::from((cx,cy))));
    }
}
impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::BLACK); //from_rgb(0.8, 0.9, 1.0));
        // graphics.draw_circle((100.0, 100.0), 75.0, Color::GREEN);
        // graphics.draw_line((10.0,10.0),(30.0,30.0),2.0, Color::RED);
        
        const RADIUS : f32 = 200.0;
        let cx : f32 = self.mouse.x; // 500.0
        let cy : f32 = self.mouse.y; // 500.0
        self.angle = (self.angle + 1.0 ) % 360.0;
        self.alpha = (self.alpha + 0.01 ) % 1.0;
        // println!("{}",self.angle);
        let rad=self.angle.to_radians();
        let x = rad.sin()*RADIUS;
        let y = rad.cos()*RADIUS;
        // graphics.draw_line((500.0,500.0),(500.0+x,500.0+y),2.0, Color::BLUE);
        graphics.draw_circle((cx+x,cy+y), 5.0, Color::from_rgba(1.0, 1.0, 1.0, self.alpha));
        self.particle.run(graphics);
        if rand::thread_rng().gen::<f32>() < 0.3 {
            self.add_particle()
        }
        match self.button_down_start {
            Some(start) => {
                for _ in 0..start.elapsed().as_millis()/250 {
                    self.add_particle();
                }
            }
            None => {}
        }
        for p in &mut self.particles {
            p.run(graphics);
        }
        self.particles.retain(|p| p.lifespan > 0.0);
        // println!("particles {}", &self.particles.len());
        helper.request_redraw();
    }
    fn on_mouse_move(
        &mut self,
        helper: &mut WindowHelper,
        position: Vector2<f32>
    ) {
        self.mouse = position;
    }
    fn on_mouse_button_down(
        &mut self,
        _helper: &mut WindowHelper,
        _button: MouseButton
    ) {
        self.button_down_start = Some(Instant::now());
    }
    fn on_mouse_button_up(
        &mut self,
        _helper: &mut WindowHelper,
        _button: MouseButton
    ) {
        self.button_down_start = None;
    }

}