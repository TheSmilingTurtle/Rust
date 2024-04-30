use image::{Luma, Rgb};

use pixie::prelude::Picture;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[derive(Clone)]
#[pyclass]
struct PyxieGrey {
    col: Luma<u8>,
}

#[pyfunction]
fn grey(val: u8) -> PyxieGrey {
    PyxieGrey {
        col: pixie::grey(val),
    }
}

#[derive(Clone)]
#[pyclass]
struct PyxieRgb {
    col: Rgb<u8>,
}

#[pyfunction]
fn rgb(r: u8, g: u8, b: u8) -> PyxieRgb {
    PyxieRgb {
        col: pixie::rgb(r, g, b),
    }
}

#[pyclass]
struct PyxiePictureGrey {
    pic: Picture<Luma<u8>, u8>,
}

#[pymethods]
impl PyxiePictureGrey {
    #[new]
    fn new() -> PyxiePictureGrey {
        PyxiePictureGrey{ pic: Picture::from_fn(0,0,|_,_| pixie::grey(0)) }
    }

    fn from_fn(&self, w: u32, h: u32, f: Py<PyAny>) -> PyxiePictureGrey {
        PyxiePictureGrey {
            pic: Picture::from_fn(w, h, |x, y| {
                Python::with_gil(|py| {
                    f.call1(py, PyTuple::new(py, vec![x, y]))
                        .unwrap()
                        .extract::<PyxieGrey>(py)
                        .unwrap()
                        .col
                })
            }),
        }
    }

    fn save(&self, path: &str) {
        self.pic.save(path)
    }
}

#[pyclass]
struct PyxiePictureRgb {
    pic: Picture<Rgb<u8>, u8>,
}

#[pymethods]
impl PyxiePictureRgb {
    #[new]
    fn new() -> PyxiePictureRgb {
        PyxiePictureRgb{ pic: Picture::from_fn(0,0,|_,_| pixie::rgb(0,0,0)) }
    }

    fn from_fn(&self, w: u32, h: u32, f: Py<PyAny>) -> PyxiePictureRgb {
        PyxiePictureRgb {
            pic: Picture::from_fn(w, h, |x, y| {
                Python::with_gil(|py| {
                    f.call1(py, PyTuple::new(py, vec![x, y]))
                        .unwrap()
                        .extract::<PyxieRgb>(py)
                        .unwrap()
                        .col
                })
            }),
        }
    }

    fn save(&self, path: &str) {
        self.pic.save(path)
    }
}

#[pymodule]
#[pyo3(name = "pyxie")]
fn my_extension(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyxieGrey>()?;
    m.add_class::<PyxieRgb>()?;
    m.add_class::<PyxiePictureGrey>()?;
    m.add_class::<PyxiePictureRgb>()?;

    m.add_function(wrap_pyfunction!(grey, m)?)?;
    m.add_function(wrap_pyfunction!(rgb, m)?)?;

    Ok(())
}