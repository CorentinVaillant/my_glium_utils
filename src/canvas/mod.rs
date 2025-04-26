use std::collections::HashMap;

use glium::{
    DrawParameters, IndexBuffer, Program, Surface, VertexBuffer, backend::Facade,
    index::PrimitiveType, winit::window::Window,
};
pub use traits::*;

use crate::mesh::vertex::Vertex;
mod traits;

#[derive(Debug, Clone, Copy)]
pub struct CanvasData {
    pub size: (f32, f32),
    pub position: (f32, f32), //%

    pub frame_nb: usize,
    pub elem_nb: usize,
    pub programs_nb: usize,

    pub window_resolution: (u32, u32),
}

pub struct CanvaElem {
    element: Box<dyn CanvasDrawable>,
    program_id: usize,
}

pub struct Canvas<'a> {
    elements: Vec<CanvaElem>,
    programs: HashMap<usize, Program>,

    data: CanvasData,

    pub draw_param: DrawParameters<'a>,
    pub z_index: f32,
}

impl<'a> Canvas<'a> {
    pub fn new(position: (f32, f32), size: (f32, f32), window_resolution: (u32, u32)) -> Self {
        Self {
            elements: vec![],
            programs: HashMap::new(),

            data: CanvasData {
                size,
                position,
                frame_nb: 0,
                elem_nb: 0,
                programs_nb: 0,
                window_resolution,
            },

            draw_param: DrawParameters::default(),
            z_index: 0.5,
        }
    }

    pub fn from_window(window: &Window) -> Self {
        Self::new(
            (0., 0.),
            (1., 1.),
            (window.inner_size().width, window.inner_size().height),
        )
    }

    pub fn set_z_index(&mut self,z:f32){
        self.z_index = z;
    }

    pub fn get_z_index(&self)->f32{
        self.z_index
    }

    pub fn get_draw_params(&'a mut self)->&'a mut DrawParameters<'a>{
        &mut self.draw_param
    }

    pub fn push_elem(&mut self, element: Box<dyn CanvasDrawable>, program_id: usize) {
        let elem = CanvaElem {
            element,
            program_id,
        };

        self.elements.push(elem);
    }

    pub fn insert_program(&mut self, program: Program, id: usize) -> Option<Program> {
        self.programs.insert(id, program)
    }
}

/*--------------------*/
/*---Drawing Canvas---*/
/*--------------------*/

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum CanvasDrawError {
    VertexBufferCreationError(glium::vertex::BufferCreationError),
    IndexBufferCreationError(glium::index::BufferCreationError),

    WrongProgrammId(usize),

    GliumDrawError(glium::DrawError),
}

impl Canvas<'_> {
    fn get_buff<F: Facade>(
        &self,
        facade: &F,
    ) -> Result<(VertexBuffer<Vertex>, IndexBuffer<u16>), CanvasDrawError> {
        let x_i = (self.data.position.0 * 2.0) - 1.0;
        let y_i = (self.data.position.1 * 2.0) - 1.0;
        let x_s = ((self.data.position.0 + self.data.size.0) * 2.0) - 1.0;
        let y_s = ((self.data.position.1 + self.data.size.1) * 2.0) - 1.0;

        let vert_buff = VertexBuffer::new(
            facade,
            &[
                [x_s, y_s].into(),
                [x_s, y_i].into(),
                [x_i, y_i].into(),
                [x_i, y_s].into(),
            ],
        )
        .map_err(CanvasDrawError::VertexBufferCreationError)?;

        let ind_buff = IndexBuffer::new(facade, PrimitiveType::TriangleFan, &[0, 1, 2, 3])
            .map_err(CanvasDrawError::IndexBufferCreationError)?;

        Ok((vert_buff, ind_buff))
    }
}

impl Drawable for Canvas<'_> {
    type DrawError = CanvasDrawError;

    fn draw<F: Facade + Sized>(
        &self,
        display: &F,
        frame: &mut glium::Frame,
    ) -> Result<(), CanvasDrawError> {
        let (v_buff, i_buff) = self.get_buff(display)?;

        for elem in self.elements.iter() {
            let mut uniforms = elem.element.canvas_uniforms();
            for uni in &mut uniforms {
                uni.add("resolution", &self.data.window_resolution);
                uni.add("canva_z", &self.z_index);
                uni.add("canva_pos", &self.data.position);
                uni.add("canva_size", &self.data.size);

                let program = self
                    .programs
                    .get(&elem.program_id)
                    .ok_or(CanvasDrawError::WrongProgrammId(elem.program_id))?;

                frame
                    .draw(&v_buff, &i_buff, program, uni, &self.draw_param)
                    .map_err(CanvasDrawError::GliumDrawError)?;
            }
        }
        Ok(())
    }
}


/*---------------------*/
/*---Updating Canvas---*/
/*---------------------*/


impl Canvas<'_>{
    pub fn update(&mut self, dt:f32){
        for elem in self.elements.iter_mut(){
            elem.element.update(&self.data, dt);
        }
        self.data.frame_nb+=1;
    }

    pub fn on_click(&mut self, abs_click_coord : (f32,f32)){
        for elem in self.elements.iter_mut(){
            if elem.element.is_absolute_coord_in(abs_click_coord){
                elem.element.on_click(abs_click_coord);}
        }
    }

    pub fn is_clicking(&self, abs_click_coord : (f32,f32))->bool{
        let rel_click_coord = (abs_click_coord.0/(self.data.window_resolution.0 as f32),abs_click_coord.1/(self.data.window_resolution.1 as f32));
        0. <= rel_click_coord.0 && rel_click_coord.0 <= self.data.size.0 &&
        0. <= rel_click_coord.1 && rel_click_coord.1 <= self.data.size.1 
    }

    pub fn on_drag(&mut self, old_pos: [f32; 2], new_pos: [f32; 2]) {
        for elem in &mut self.elements {
            if elem.element.is_absolute_coord_in(old_pos.into()) {
                elem.element.on_drag(old_pos, new_pos);
            }
        }
    }

    pub fn on_window_moved(&mut self, new_pos: (f32, f32)) {
        for elem in self.elements.iter_mut(){
            elem.element.on_window_moved(new_pos);
        }
    }
    
    pub fn on_window_resized(&mut self, new_size: (u32, u32)) {
        for elem in self.elements.iter_mut(){
            elem.element.on_window_resized(new_size);
        }
    }
}