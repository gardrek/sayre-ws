pub trait RenderTarget {
    fn get_buffer(&mut self) -> (&mut [u8], usize);

    fn get_width(&mut self) -> (&mut [u8], usize);
}

pub fn line(buffer: &mut [u8], run: usize, x0: f64, y0: f64, x1: f64, y1: f64) {
    eprintln!(
        "line({:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
        buffer, run, x0, y0, x1, y1
    );
    todo!()
}
