use flatbox_core::math::{
    glm, 
    transform::Transform,
};
use flatbox_render::{
    error::RenderError, 
    hal::{
        shader::GraphicsPipeline, 
        GlInitFunction,
    }, 
    pbr::{
        camera::Camera, 
        material::Material, 
        mesh::{Mesh, MeshType, Vertex}, 
        model::Model, 
        texture::{Order, Texture},
    }, 
    renderer::{ClearCommand, DrawModelCommand, PrepareModelCommand, RenderCameraCommand, Renderer as FlatboxRenderer, WindowExtent},
};
use serde::{Deserialize, Serialize};
use tiny_skia::Pixmap;

#[derive(Default)]
pub struct UiCamera {
    cam: Camera,
    transform: Transform,
}

pub struct RenderSurface {
    width: f32,
    height: f32,
    material: UiMaterial,
    model: Model,
    transform: Transform,
}

impl RenderSurface {
    fn new(screen_width: u32, screen_height: u32) -> RenderSurface {
        RenderSurface {
            width: screen_width as f32,
            height: screen_height as f32,
            material: UiMaterial::default(),
            model: Model::new(
                MeshType::Generic, 
                Mesh::new(
                    &[
                        Vertex { 
                            position: glm::vec3(-1.0,1.0,0.0), 
                            normal: glm::vec3(0.0, 0.0, -1.0), 
                            texcoord: glm::vec2(0.0, 0.0) 
                        },
                        Vertex { 
                            position: glm::vec3(-1.0,-1.0,0.0), 
                            normal: glm::vec3(0.0, 0.0, -1.0), 
                            texcoord: glm::vec2(0.0, 1.0) 
                        },
                        Vertex { 
                            position: glm::vec3(1.0,-1.0,0.0), 
                            normal: glm::vec3(0.0, 0.0, -1.0), 
                            texcoord: glm::vec2(1.0, 1.0) 
                        },
                        Vertex { 
                            position: glm::vec3(1.0,1.0,0.0), 
                            normal: glm::vec3(0.0, 0.0, -1.0), 
                            texcoord: glm::vec2(1.0, 0.0) 
                        },
                    ], 
                    &[0,1,3,3,1,2],
                    &[],
                ),
            ),
            transform: Transform::identity(),
        }
    }

    fn set_size(&mut self, screen_width: f32, screen_height: f32) {
        self.model.mesh.as_mut().unwrap().vertex_data = vec![
            Vertex { 
                position: glm::vec3(-1.0,1.0,0.0), 
                normal: glm::vec3(0.0, 0.0, -1.0), 
                texcoord: glm::vec2(0.0, 0.0) 
            },
            Vertex { 
                position: glm::vec3(-1.0,-1.0,0.0), 
                normal: glm::vec3(0.0, 0.0, -1.0), 
                texcoord: glm::vec2(0.0, 1.0) 
            },
            Vertex { 
                position: glm::vec3(1.0,-1.0,0.0), 
                normal: glm::vec3(0.0, 0.0, -1.0), 
                texcoord: glm::vec2(1.0, 1.0) 
            },
            Vertex { 
                position: glm::vec3(1.0,1.0,0.0), 
                normal: glm::vec3(0.0, 0.0, -1.0), 
                texcoord: glm::vec2(1.0, 0.0) 
            },
        ];
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct UiMaterial {
    pub surface: Texture,
}

#[typetag::serde]
impl Material for UiMaterial {
    fn vertex_shader() ->  &'static str {
        include_str!("shaders/uimaterial.vs")
    }

    fn fragment_shader() ->  &'static str {
        include_str!("shaders/uimaterial.fs")
    }

    fn setup_pipeline(&self, pipeline: &GraphicsPipeline) {
        pipeline.set_int("material.surface", 0);
        self.surface.activate(Order::Texture0);
    }
}

pub struct Renderer {
    pub inner: FlatboxRenderer,
    pub surface: RenderSurface,
    pub camera: UiCamera,
}

impl Renderer {
    pub fn new<F: GlInitFunction>(init_function: F, screen_width: u32, screen_height: u32) -> Renderer {
        let mut renderer = FlatboxRenderer::init(init_function);
        renderer.bind_material::<UiMaterial>();

        Renderer {
            inner: renderer,
            surface: RenderSurface::new(screen_width, screen_height),
            camera: UiCamera::default(),
        }
    }

    pub fn render(&mut self, pixmap: Pixmap) -> Result<(), RenderError> {
        self.surface.material.surface = Texture::new_from_raw(pixmap.data(), pixmap.width(), pixmap.height(), None)?;
        self.inner.execute(&mut ClearCommand(0.0, 0.5, 1.0))?;
        self.inner.execute(&mut RenderCameraCommand::<UiMaterial>::new(&mut self.camera.cam, &self.camera.transform))?;
        self.inner.execute(&mut PrepareModelCommand::new(&mut self.surface.model, &self.surface.material))?;
        self.inner.execute(&mut DrawModelCommand::new(&self.surface.model, &self.surface.material, &self.surface.transform))?;
    
        Ok(())
    }

    pub fn get_extent(&self) -> WindowExtent {
        self.inner.extent()
    }

    pub fn resize(&mut self, extent: WindowExtent) {
        self.inner.set_extent(extent);
        self.surface.set_size(extent.width, extent.height);
    }
}

