use crate::error::Error;
use crate::picture::Colour;

use super::Picture;

pub struct PictureBuilder {
    bounds: Option<(usize, usize)>,
    function: Option<fn(usize, usize) -> Colour>,
}

impl PictureBuilder {
    pub(crate) fn new() -> Self {
        PictureBuilder {
            bounds: None,
            function: None,
        }
    }

    fn bounds(mut self, x: usize, y: usize) -> Self {
        self.bounds = Some((x, y));

        self
    }

    fn add_fn(mut self, function: fn(usize, usize) -> Colour) -> Self {
        self.function = Some(function);

        self
    }

    fn from_fn(mut self, function: fn(usize, usize) -> Colour) -> Result<Picture, Error> {
        self.add_fn(function).calculate()
    }

    fn from_fn_par(
        mut self,
        thread_count: usize,
        function: fn(usize, usize) -> Colour,
    ) -> Result<Picture, Error> {
        self.add_fn(function).calculate_par(thread_count)
    }

    fn calculate(self) -> Result<Picture, Error> {
        match (self.bounds, self.function) {
            (None, _) => return Err(Error::NoBounds),
            (_, None) => return Err(Error::NoFunction),
            (Some(bounds), Some(function)) => {
                let mut temp: Picture = Picture {
                    bounds: bounds,
                    pixels: Vec::with_capacity(bounds.0 * bounds.1),
                };

                for i in 0..bounds.0 {
                    for j in 0..bounds.1 {
                        temp.pixels[bounds.0 * j + i] = function(i, j);
                    }
                }

                Ok(temp)
            }
        }
    }

    fn calculate_par(&self, thread_count: usize) -> Result<Picture, Error> {
        match (self.bounds, self.function) {
            (None, _) => return Err(Error::NoBounds),
            (_, None) => return Err(Error::NoFunction),
            (Some(bounds), Some(function)) => {
                let res;

                std::thread::scope(|s| {
                    let length = std::cmp::max(1, bounds.0 / thread_count);
                    let mut thread_vec: Vec<std::thread::ScopedJoinHandle<Vec<Colour>>>;

                    for start in (0..thread_count).step_by(length) {
                        thread_vec.push(s.spawn(|| {
                            let mut temp: Vec<Colour> = Vec::with_capacity(length * bounds.1);

                            for i in start..(start + length) {
                                for j in 0..bounds.1 {
                                    temp[bounds.0 * j + i] = function(i, j);
                                }
                            }

                            temp
                        }));
                    }

                    res = thread_vec
                        .into_iter()
                        .map(|a| a.join())
                        .collect::<Result<Vec<_>, _>>()
                });

                match res {
                    Ok(out) => Ok(Picture {
                        bounds: bounds,
                        pixels: out.into_iter().flatten().collect(),
                    }),
                    Err(_) => Err(Error::ThreadPaniced)
                }
            }
        }
    }
}
