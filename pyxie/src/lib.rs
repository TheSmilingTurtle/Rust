use pyo3::{prelude::*, methods::OkWrap};
use pixie::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), std::io::Error> {
        let c = Colour::new("Grey", vec![0]);

        dbg!(c);

        Ok(())
    }
}

#[pyclass]
struct PyxieColour{
    colour: Colour
}

#[pyfunction]
fn grey(val: u8) -> PyResult<PyxieColour> {
    Ok( PyxieColour{colour: Colour::new("Grey", vec![val]).unwrap()} )
}

#[pyfunction]
fn rgb(r: u8, g: u8, b: u8) -> PyResult<PyxieColour> {
    Ok(PyxieColour{colour: Colour::new("Rgb", vec![r, g, b]).unwrap()} )
}

#[pyclass]
struct PyxiePicture{
    picture: Picture
}

#[pyclass]
struct PyxiePictureBuilder{
    picture_builder: pixie::picture::PictureBuilder
}

#[pymethods]
impl PyxiePicture {
    #[new]
    fn new() -> Self {
        PyxiePicture{picture: Picture{bounds: (0,0), pixels: vec![]}}
    }

    #[staticmethod]
    fn build() -> PyxiePictureBuilder {
        PyxiePictureBuilder::new()
    }
    
    fn save(&self, path: &str) -> PyResult<()> {
        let _ = self.picture.save(path);

        Ok(())
    }
}

#[pymethods]
impl PyxiePictureBuilder {
    #[new]
    fn new() -> Self {
        PyxiePictureBuilder{picture_builder: Picture::build()}
    }

    pub fn bounds(&mut self, x: usize, y: usize) -> Self {
        self.picture_builder.bounds = Some((x, y));

        *self
    }

    pub fn add_fn(&mut self, function: Py<PyAny>) -> Self {
        self.picture_builder.function = Some(|| function);

        *self
    }

    pub fn from_fn(&mut self, function: Py<PyAny>) -> PyResult<PyxiePicture> {
        self.add_fn(function).calculate()
    }

    pub fn from_fn_par(
        &mut self,
        function: Py<PyAny>,
        thread_count: usize,
    ) -> PyResult<PyxiePicture> {
        self.add_fn(function).calculate_par(thread_count)
    }

    pub fn calculate(&self) -> PyResult<PyxiePicture> {
    }

    pub fn calculate_par(&self, thread_count: usize) -> PyResult<PyxiePicture> {
    }

}