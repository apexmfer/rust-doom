use super::lights::Lights;
use engine::{
    Analog2d, DependenciesFrom, Entities, EntityId, Gesture, InfallibleSystem, Input, MouseButton,
    Projection, Projections, RenderPipeline, Scancode, Tick, Transforms, Window,
    Uniforms, Meshes 
};
use log::{debug, error, info, warn};
use math::prelude::*;
use math::{vec3, Deg, Euler, Pnt3f, Quat, Rad, Sphere, Trans3, Vec3f};
use super::vertex::{SkyVertex, SpriteVertex, StaticVertex};
use vec_map::VecMap;

use super::world::{World, WorldBuilder};
use std::time::Instant;
use super::scene_shaders::{SceneShaders, SceneMaterials};


pub struct Scene {
    root: EntityId,
    objects: Vec<EntityId>,
    
    removed: Vec<usize>,
 

    start_pos: Pnt3f,
    start_yaw: Rad<f32>,
    lights: Lights,
    volume: World,
}




#[derive(DependenciesFrom)]
pub struct Dependencies<'context> {

    //these require pluck ??
   // bindings: &'context Bindings,
    config: &'context Config,



    tick: &'context Tick,
    window: &'context Window,
    input: &'context Input,

    
    entities: &'context mut Entities,
    transforms: &'context mut Transforms,
    projections: &'context mut Projections,
    uniforms: &'context mut Uniforms,
    meshes: &'context mut Meshes,
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



            
            /*let mut scene = Scene {
                objects: Vec::new()
            };*/


            
             Builder::build(&mut deps);  //returns a scene 

           


            
    }

    fn update(&mut self, deps: Dependencies) {




    }

    fn teardown(&mut self, deps: Dependencies) {
      //  deps.entities.remove(self.id);
    }
}


struct Indices {
    wall: Vec<u32>,
    flat: Vec<u32>,
    sky: Vec<u32>,
    decor: Vec<u32>,
}


//consider moving builder to elsewhere 
struct Builder<'a> {
    materials: &'a SceneMaterials,

    lights: Lights,
    start_pos: Pnt3f,
    start_yaw: Rad<f32>,

    static_vertices: Vec<StaticVertex>,
    sky_vertices: Vec<SkyVertex>,
    decor_vertices: Vec<SpriteVertex>,

    object_indices: VecMap<Indices>,

    num_wall_quads: usize,
    num_floor_polys: usize,
    num_ceil_polys: usize,
    num_sky_wall_quads: usize,
    num_sky_floor_polys: usize,
    num_sky_ceil_polys: usize,
    num_decors: usize,
}

impl<'a> Builder<'a> {
    fn build(deps: &mut Dependencies) -> Result<Scene> {
        info!("Building new scene...");

        let start_time = Instant::now();
        let root = deps.entities.add_root("scene_root");

        let mut objects = Vec::new();
        let world = deps.entities.add(root, "world")?;
        deps.transforms.attach_identity(world);
        objects.extend((0..deps.wad.analysis.num_objects()).map(|i_object| {
            let entity = deps
                .entities
                .add(
                    world,
                    if i_object == 0 {
                        "static_object"
                    } else {
                        "dynamic_object"
                    },
                )
                .expect("add entity to world");
            deps.transforms.attach_identity(entity);
            entity
        }));

        let mut builder = Builder {
            materials: deps.game_shaders.level_materials(),

            lights: Lights::new(),
            start_pos: Pnt3f::origin(),
            start_yaw: Rad(0.0f32),

            static_vertices: Vec::with_capacity(16_384),
            sky_vertices: Vec::with_capacity(16_384),
            decor_vertices: Vec::with_capacity(16_384),

            object_indices: VecMap::new(),

            num_wall_quads: 0,
            num_floor_polys: 0,
            num_ceil_polys: 0,
            num_sky_wall_quads: 0,
            num_sky_floor_polys: 0,
            num_sky_ceil_polys: 0,
            num_decors: 0,
        };

        info!("Walking level...");
      /*   let volume = {
            let mut world_builder = WorldBuilder::new(&objects);
            deps.wad.walk(&mut builder.chain(&mut world_builder));
            world_builder.build()
        };
*/
        //we need to add stuff to the scene LIKE worldbuilder+wad does but we need to do it manually 




        info!(
            "Level built in {:.2}ms:\n\
             \tnum_wall_quads = {}\n\
             \tnum_floor_polys = {}\n\
             \tnum_ceil_polys = {}\n\
             \tnum_sky_wall_quads = {}\n\
             \tnum_sky_floor_polys = {}\n\
             \tnum_sky_ceil_polys = {}\n\
             \tnum_decors = {}\n\
             \tnum_static_tris = {}\n\
             \tnum_sky_tris = {}\n\
             \tnum_sprite_tris = {}",
            start_time.elapsed().f64_seconds() * 1000.0,
            builder.num_wall_quads,
            builder.num_floor_polys,
            builder.num_ceil_polys,
            builder.num_sky_wall_quads,
            builder.num_sky_floor_polys,
            builder.num_sky_ceil_polys,
            builder.num_decors,
            builder
                .object_indices
                .values()
                .map(|indices| indices.wall.len() + indices.flat.len())
                .sum::<usize>()
                / 3,
            builder
                .object_indices
                .values()
                .map(|indices| indices.sky.len())
                .sum::<usize>()
                / 3,
            builder
                .object_indices
                .values()
                .map(|indices| indices.decor.len())
                .sum::<usize>()
                / 3,
        );



           /*//see level ln 424 
              info!("Creating static meshes and models...");

              //see renderer ln 100 
              //pipe.models.access().iter().enumerate()

              deps
              .meshes 
              .add( deps.window, deps.entities, root,  "menu_background" )
              .immutable(&builder.static_vertices)
              .build_unindexed();

                */


        info!("Creating static meshes and models...");
        let global_static_mesh = deps
            .meshes
            .add(deps.window, deps.entities, root, "global_world_static_mesh")
            .immutable(&builder.static_vertices)?
            .build_unindexed()?;

        let global_sky_mesh = deps
            .meshes
            .add(deps.window, deps.entities, root, "global_world_sky_mesh")
            .immutable(&builder.sky_vertices)?
            .build_unindexed()?;

        let global_decor_mesh = deps
            .meshes
            .add(deps.window, deps.entities, root, "global_world_decor_mesh")
            .immutable(&builder.decor_vertices)?
            .build_unindexed()?;

        for (id, indices) in &builder.object_indices {
            let object = objects[id];
            if !indices.flat.is_empty() {
                let entity = deps.entities.add(object, "flats")?;
                let mesh = deps
                    .meshes
                    .add(deps.window, deps.entities, entity, "object_flats_mesh")
                    .shared(global_static_mesh)
                    .immutable_indices(&indices.flat)?
                    .build()?;
                deps.transforms.attach_identity(entity);
                deps.render
                    .attach_model(entity, mesh, builder.materials.flats.material);
            }

            if !indices.wall.is_empty() {
                let entity = deps.entities.add(object, "walls")?;
                let mesh = deps
                    .meshes
                    .add(deps.window, deps.entities, entity, "object_walls_mesh")
                    .shared(global_static_mesh)
                    .immutable_indices(&indices.wall)?
                    .build()?;
                deps.transforms.attach_identity(entity);
                deps.render
                    .attach_model(entity, mesh, builder.materials.walls.material);
            }

            if !indices.decor.is_empty() {
                let entity = deps.entities.add(object, "decor")?;
                let mesh = deps
                    .meshes
                    .add(deps.window, deps.entities, entity, "object_decor_mesh")
                    .shared(global_decor_mesh)
                    .immutable_indices(&indices.decor)?
                    .build()?;
                deps.transforms.attach_identity(entity);
                deps.render
                    .attach_model(entity, mesh, builder.materials.decor.material);
            }

            if !indices.sky.is_empty() {
                let entity = deps.entities.add(object, "sky")?;
                let mesh = deps
                    .meshes
                    .add(deps.window, deps.entities, entity, "object_sky_mesh")
                    .shared(global_sky_mesh)
                    .immutable_indices(&indices.sky)?
                    .build()?;
                deps.transforms.attach_identity(entity);
                deps.render
                    .attach_model(entity, mesh, builder.materials.sky);
            }
        }

        Ok(Scene {
            root,
            volume,
            objects,
          //  triggers: deps.wad.analysis.take_triggers(),
            removed: Vec::with_capacity(128),
           // effects: VecMap::new(),
            start_pos: builder.start_pos,
            start_yaw: builder.start_yaw,
            lights: builder.lights,
          //  exit_triggered: false,
           // level_changed: true,
        })


    


    }