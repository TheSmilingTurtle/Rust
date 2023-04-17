use pyo3::{prelude::*, types::PyTuple, exceptions::{PyAttributeError, PyException}};
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

#[derive(Clone)]
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
    bounds: Option<(usize, usize)>,
    function: Option<Py<PyAny>>
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
        PyxiePictureBuilder{
            bounds: None,
            function: None
        }
    }

    pub fn bounds(&mut self, x: usize, y: usize) {
        self.bounds = Some((x, y));
    }

    pub fn add_fn(&mut self, function: Py<PyAny>) {
        self.function = Some(function);
    }

    pub fn from_fn(&mut self, function: Py<PyAny>) -> PyResult<PyxiePicture> {
        self.add_fn(function);
        self.calculate()
    }

    fn calculate_range(&self, range: (usize, usize)) -> Vec<PyxieColour> {
        match (self.bounds, &self.function) {
            (None, _) => panic!("Bounds Missing"),
            (_, None) => panic!("Function Missing"),
            (Some(bounds), Some(function)) => {
                let mut temp = Vec::with_capacity((range.1 - range.0) * bounds.1);

                for i in range.0..range.1 {
                    for j in 0..bounds.1 {
                        let return_val = Python::with_gil(|py|
                                            function.call1(py, PyTuple::new(py, vec![j, i]))?.extract::<PyxieColour>(py)
                                            ).unwrap();
                        temp.push(return_val);
                    }
                }

                temp
            }
        }
    }

    pub fn calculate(&self) -> PyResult<PyxiePicture> {
        match self.bounds {
            Some(bounds) => Ok( PyxiePicture{
                    picture: Picture{
                        bounds,
                        pixels: self.calculate_range((0,bounds.0)).iter().map(|x| x.colour).collect::<Vec<_>>(),
                    }}),
            None => Err(PyAttributeError::new_err("Bounds Missing"))
        }
    }
}

#[pymodule]
fn my_extension(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyxieColour>()?;
    m.add_class::<PyxiePicture>()?;

    m.add_function(wrap_pyfunction!(grey, m)?)?;
    m.add_function(wrap_pyfunction!(rgb, m)?)?;

    Ok(())
}
