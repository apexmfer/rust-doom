use super::errors::{ErrorKind, Result};
 
use super::hud::{Bindings as HudBindings, Hud};
  
use super::SHADER_ROOT;
use engine::type_list::Peek;
use engine::{
    Context, ContextBuilder, Entities, FrameTimers, Input, Materials, Meshes, Projections,
    RenderPipeline, Renderer, ShaderConfig, Shaders, System, TextRenderer, Tick, TickConfig,
    Transforms, Uniforms, Window, WindowConfig,
};
use failchain::ResultExt;
use std::marker::PhantomData;
use std::path::PathBuf;

pub trait Menu {
    fn run(self) -> !;
    fn destroy(&mut self) -> Result<()>;
  
}

#[derive(Clone)]
pub struct MenuConfig {
    pub wad_file: PathBuf,
    pub metadata_file: PathBuf,
    pub fov: f32,
    pub width: u32,
    pub height: u32,
    pub version: &'static str,
    pub initial_level_index: usize,
}

pub fn create(config: &MenuConfig) -> Result<impl Menu> {
    let context = (|| {
        ContextBuilder::new()
            // Engine configs and systems.
            .inject(TickConfig {
                timestep: 1.0 / 60.0,
            })
            .inject(WindowConfig {
                width: config.width,
                height: config.height,
                title: format!("Rusty Doom v{}", config.version),
            })
            .inject(ShaderConfig {
                root_path: SHADER_ROOT.into(),
            })
            .system(Tick::bind())?
            .system(FrameTimers::bind())?
            .system(Window::bind())?
            .system(Input::bind())?
            .system(Entities::bind())?
            .system(Transforms::bind())?
            .system(Projections::bind())?
            .system(Shaders::bind())?
            .system(Uniforms::bind())?
            .system(Meshes::bind())?
            .system(Materials::bind())?
            .system(RenderPipeline::bind())?
            .system(TextRenderer::bind())?
 
 
            .build()
    })()
    .chain_err(|| ErrorKind("during setup".to_owned()))?;

    println!("menu built ");

    Ok(MenuImpl::new(context))
}

struct MenuImpl<ContextT>
where
    ContextT: Context,
{
    context: Option<ContextT> 
}

impl< ContextT> MenuImpl< ContextT>
where
    ContextT: Context,
{
    fn new(context: ContextT) -> Self {
        Self {
            context: Some(context) 
        }
    }
}

impl <ContextT> Menu for MenuImpl<ContextT>
where
    ContextT: Context,
{
    fn run(mut self) -> ! {
        self.context.take().unwrap().run()
    }

   
/*
    fn load_level(&mut self, level_index: usize) -> Result<()> {
        let context = self.context.as_mut().unwrap();
        let wad = context.peek_mut();
        wad.change_level(level_index);
        context
            .step()
            .chain_err(|| ErrorKind("during load_level first step".to_owned()))?;
        context
            .step()
            .chain_err(|| ErrorKind("during load_level second step".to_owned()))?;
        Ok(())
    }
*/
    fn destroy(&mut self) -> Result<()> {
        if let Some(context) = self.context.as_mut() {
            context
                .destroy()
                .chain_err(|| ErrorKind("during explicit destroy".to_owned()))?;
        }
        Ok(())
    }
}

impl< ContextT> Drop for MenuImpl< ContextT>
where
    ContextT: Context ,
{
    fn drop(&mut self) {
        if let Some(mut context) = self.context.take() {
            let _ = context.destroy();
        }
    }
}
