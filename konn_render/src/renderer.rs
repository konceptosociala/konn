use std::collections::{hash_map::Entry, HashMap};

use flatbox_core::{logger::error, math::{
    glm,
    transform::Transform,
}};
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
use tiny_skia::{Paint, Pixmap, Shader, Transform as SkiaTransform};

use crate::{
    widgets::{Size, WidgetId},
    Color, Rect,
};

pub struct Renderer {
    inner: FlatboxRenderer,
    camera: UiCamera,
    canvases: HashMap<WidgetId, Canvas>,
}

impl Renderer {
    pub fn new(init_function: impl GlInitFunction) -> Renderer {
        let mut renderer = FlatboxRenderer::init(init_function);
        renderer.bind_material::<UiMaterial>();

        Renderer {
            inner: renderer,
            camera: UiCamera::default(),
            canvases: HashMap::new(),
        }
    }

    pub fn add_canvas(&mut self, id: WidgetId, canvas_size: Size) {
        let screen_size = self.get_extent();

        if let Entry::Vacant(e) = self.canvases.entry(id) {
            let canvas = Renderer::new_canvas(canvas_size, screen_size);
            e.insert(canvas);
        } else {
            error!("Widget with id `{id}` already exists");
        }
    }

    pub fn get_canvas(&mut self, id: &WidgetId) -> Option<&mut Canvas> {
        self.canvases.get_mut(id)
    }

    pub fn clear(&mut self) {
        self.inner.execute(&mut ClearCommand(0.0, 0.5, 1.0)).unwrap();
    }

    pub fn render(&mut self, id: &WidgetId) -> Result<(), RenderError> {
        let canvas = self.canvases.get_mut(id).unwrap_or_else(|| {
            panic!("Canvas for widget `{id}` not found");
        });

        self.inner.execute(&mut RenderCameraCommand::<UiMaterial>::new(
            &mut self.camera.cam, 
            &self.camera.transform,
        ))?;

        self.inner.execute(&mut PrepareModelCommand::new(
            &mut canvas.model, 
            &canvas.material,
        ))?;

        self.inner.execute(&mut DrawModelCommand::new(
            &canvas.model, 
            &canvas.material, 
            &canvas.transform,
        ))?;
    
        Ok(())
    }

    pub fn get_extent(&self) -> WindowExtent {
        self.inner.extent()
    }

    pub fn resize(&mut self, extent: WindowExtent) {
        self.inner.set_extent(extent);
    }

    fn new_canvas(size: Size, screen_size: WindowExtent) -> Canvas {
        let width = size.width as f32;
        let height = size.height as f32;

        Canvas {
            pixmap: Pixmap::new(size.width, size.height).unwrap_or_else(|| {
                panic!("Invalid canvas size `{width}x{height}`");
            }),
            material: UiMaterial::default(),
            model: Model::new(
                MeshType::Generic, 
                Mesh::new(
                    &[
                        Vertex { 
                            position: glm::vec3(-1.0*width/screen_size.height,1.0*height/screen_size.height,0.0), 
                            normal: glm::vec3(0.0, 0.0, -1.0), 
                            texcoord: glm::vec2(0.0, 0.0) 
                        },
                        Vertex { 
                            position: glm::vec3(-1.0*width/screen_size.height,-1.0*height/screen_size.height,0.0), 
                            normal: glm::vec3(0.0, 0.0, -1.0), 
                            texcoord: glm::vec2(0.0, 1.0) 
                        },
                        Vertex { 
                            position: glm::vec3(1.0*width/screen_size.height,-1.0*height/screen_size.height,0.0), 
                            normal: glm::vec3(0.0, 0.0, -1.0), 
                            texcoord: glm::vec2(1.0, 1.0) 
                        },
                        Vertex { 
                            position: glm::vec3(1.0*width/screen_size.height,1.0*height/screen_size.height,0.0), 
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
}

pub struct Canvas {
    pixmap: Pixmap,
    material: UiMaterial,
    model: Model,
    transform: Transform,
}

impl Canvas {
    pub fn rect(&mut self, rect: Rect, color: Color) {
        let paint = Paint {
            anti_alias: true,
            shader: Shader::SolidColor(color),
            ..Default::default()
        };

        self.pixmap.fill_rect(
            rect, 
            &paint, 
            SkiaTransform::identity(), 
            None,
        )
    }

    pub fn clear(&mut self, color: Color) {
        self.pixmap = Pixmap::new(self.pixmap.width(), self.pixmap.height()).unwrap();
        self.pixmap.fill(color);
    }

    pub fn resize(&mut self, new_size: Size) {
        self.pixmap = Pixmap::new(new_size.width, new_size.height).unwrap();
    }

    pub fn submit(&mut self) {
        self.material.surface = Texture::new_from_raw(
            self.pixmap.data(), 
            self.pixmap.width(), 
            self.pixmap.height(), 
            None
        ).expect("Cannot submit canvas texture");
    }
}

#[derive(Default)]
pub struct UiCamera {
    cam: Camera,
    transform: Transform,
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
