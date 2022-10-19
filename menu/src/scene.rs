
use engine::{
    Analog2d, DependenciesFrom, Entities, EntityId, Gesture, InfallibleSystem, Input, MouseButton,
    Projection, Projections, RenderPipeline, Scancode, Tick, Transforms, Window,
};
use log::error;
use math::prelude::*;
use math::{vec3, Deg, Euler, Pnt3f, Quat, Rad, Sphere, Trans3, Vec3f};
use std::f32::consts::FRAC_PI_2;






pub struct Scene {
   // root: EntityId,
    objects: Vec<EntityId>,
   // triggers: Vec<Trigger>,
   // removed: Vec<usize>,
   // effects: VecMap<MoveEffect>,
  //  exit_triggered: bool,
  //  level_changed: bool,

 //   start_pos: Pnt3f,
  //  start_yaw: Rad<f32>,
   // lights: Lights,
   // volume: World,
}




#[derive(DependenciesFrom)]
pub struct Dependencies<'context> {
    bindings: &'context Bindings,
    config: &'context Config,

    tick: &'context Tick,
    window: &'context Window,
    input: &'context Input,
    entities: &'context mut Entities,
    transforms: &'context mut Transforms,
    projections: &'context mut Projections,
    render: &'context mut RenderPipeline,

  //  level: &'context mut Level,
}






impl Scene {
     
}




pub struct Bindings {
    pub movement: Analog2d,
    pub look: Analog2d,
    pub jump: Gesture,
    pub fly: Gesture,
    pub clip: Gesture,
    pub push: Gesture,
    pub shoot: Gesture,
}

impl Default for Bindings {
    fn default() -> Bindings {
        Bindings {
            movement: Analog2d::Gestures {
                x_positive: Gesture::KeyHold(Scancode::D),
                x_negative: Gesture::KeyHold(Scancode::A),
                y_positive: Gesture::KeyHold(Scancode::S),
                y_negative: Gesture::KeyHold(Scancode::W),
                step: 1.0,
            },
            look: Analog2d::Sum {
                analogs: vec![
                    Analog2d::Gestures {
                        x_positive: Gesture::KeyHold(Scancode::Right),
                        x_negative: Gesture::KeyHold(Scancode::Left),
                        y_positive: Gesture::KeyHold(Scancode::Down),
                        y_negative: Gesture::KeyHold(Scancode::Up),
                        step: 0.015,
                    },
                    Analog2d::Mouse {
                        sensitivity: 0.0015,
                    },
                ],
            },
            jump: Gesture::KeyHold(Scancode::Space),
            push: Gesture::KeyTrigger(Scancode::E),
            shoot: Gesture::ButtonTrigger(MouseButton::Left),
            fly: Gesture::KeyTrigger(Scancode::F),
            clip: Gesture::KeyTrigger(Scancode::C),
        }
    }
}




pub struct Config {
    move_force: f32,
    spring_const_p: f32,
    spring_const_d: f32,
    radius: f32,
    height: f32,
    air_drag: f32,
    ground_drag: f32,
    friction: f32,

    fov: Deg<f32>,
    near: f32,
    far: f32,
    aspect_ratio_correction: f32,

    camera_height: f32,
}


impl Default for Config {
    fn default() -> Self {
        Config {
            move_force: 60.0,
            spring_const_p: 200.0,
            spring_const_d: 22.4,
            radius: 0.19,
            height: 0.21,
            air_drag: 0.02,
            ground_drag: 0.7,
            friction: 30.0,

            fov: Deg(65.0),
            near: 0.01,
            far: 100.0,
            aspect_ratio_correction: 1.2,

            camera_height: 0.12,
        }
    }
}





impl<'context> InfallibleSystem<'context> for Scene {
    type Dependencies = Dependencies<'context>;

    fn debug_name() -> &'static str {
        "scene"
    }

    fn create(deps: Dependencies) -> Scene {


            let camera_entity = deps.entities.add_root("camera");
            deps.transforms.attach_identity(camera_entity);
            

            deps.transforms.attach(
                camera_entity,
                Trans3 {
                    disp: Vec3f::new(0.0, deps.config.camera_height, 0.0),
                    ..Trans3::one()
                },
            );


            deps.projections.attach(
                camera_entity,
                Projection {
                    fov: deps.config.fov.into(),
                    aspect_ratio: deps.window.aspect_ratio() * deps.config.aspect_ratio_correction,
                    near: deps.config.near,
                    far: deps.config.far,
                },
            );
            deps.render.set_camera(camera_entity);



            
            let mut scene = Scene {
                objects: Vec::new()
            };

            scene
    }

    fn update(&mut self, deps: Dependencies) {




    }

    fn teardown(&mut self, deps: Dependencies) {
      //  deps.entities.remove(self.id);
    }
}