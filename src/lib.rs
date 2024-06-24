pub mod vertex;
pub mod vertex_array;
pub mod atomic;
pub mod program;
pub mod shader;
pub mod graphics_error;
pub mod graphics_pointer;
pub mod uniform;
pub mod font;
pub mod textures;
pub mod a_or_b;
pub mod model;
pub mod bundled_model;
pub mod bundled_mesh;
pub mod frame_buffer;
pub mod render_buffer;
pub mod ui;
pub mod ffi;

pub mod render_texture;

#[cfg(test)]
mod tests {
    use bumpalo::Bump;

    #[test]
    fn it_works() {
        let expression = "5  * 4";

        let ctx = aftermath::Context::new();
        let mut bump = Bump::new();
        let expr = aftermath::Expr::parse(&mut bump, expression, &[]).unwrap();
        let e = ctx.eval(expr).unwrap();
        println!("{:?}", e.norm());
    }
}