use crate::error::Error;
use crate::picture::Colour;

use super::Picture;

#[derive(Debug)]
pub struct PictureBuilder {
    pub bounds: Option<(usize, usize)>,
    pub function: Option<fn(usize, usize) -> Colour>,
}

impl PictureBuilder {
    pub(crate) fn new() -> Self {
        PictureBuilder {
            bounds: None,
            function: None,
        }
    }

    pub fn bounds(mut self, x: usize, y: usize) -> Self {
        self.bounds = Some((x, y));

        self
    }

    pub fn add_fn(mut self, function: fn(usize, usize) -> Colour) -> Self {
        self.function = Some(function);

        self
    }

    pub fn from_fn(self, function: fn(usize, usize) -> Colour) -> Result<Picture, Error> {
        self.add_fn(function).calculate()
    }

    pub fn from_fn_par(
        self,
        function: fn(usize, usize) -> Colour,
        thread_count: usize,
    ) -> Result<Picture, Error> {
        self.add_fn(function).calculate_par(thread_count)
    }

    pub fn calculate(self) -> Result<Picture, Error> {
        match (self.bounds, self.function) {
            (None, _) => Err(Error::NoBounds),
            (_, None) => Err(Error::NoFunction),
            (Some(bounds), Some(function)) => {
                let mut temp: Picture = Picture {
                    bounds,
                    pixels: Vec::with_capacity(bounds.0 * bounds.1),
                };

                for i in 0..bounds.0 {
                    for j in 0..bounds.1 {
                        temp.pixels.push(function(j, i));
                    }
                }

                Ok(temp)
            }
        }
    }

    pub fn calculate_par(self, thread_count: usize) -> Result<Picture, Error> {
        match (self.bounds, self.function) {
            (None, _) => Err(Error::NoBounds),
            (_, None) => Err(Error::NoFunction),
            (Some(bounds), Some(function)) => {
                let mut temp: Picture = Picture {
                    bounds,
                    pixels: Vec::with_capacity(bounds.0 * bounds.1),
                };

                if thread_count == 0 {
                    return Err(Error::ZeroThreads);
                }

                std::thread::scope(|s| {
                    let length = (bounds.0 / thread_count).clamp(1, bounds.0);
                    let mut thread_vec: Vec<std::thread::ScopedJoinHandle<Vec<Colour>>> = Vec::with_capacity(thread_count);

                    for start in 0..thread_count {
                        thread_vec.push(s.spawn(move || {
                            let mut t: Vec<Colour> = Vec::with_capacity(length * bounds.1);

                            for i in (start * length)..((start + 1) * length) {
                                for j in 0..bounds.1 {
                                    t.push(function(j, i));
                                }
                            }

                            t
                        }));
                    }

                    if thread_count*length < bounds.0 {
                        thread_vec.push(s.spawn(move || {
                            let mut t: Vec<Colour> = Vec::with_capacity(length * bounds.1);

                            for i in (thread_count*length)..(bounds.0) {
                                for j in 0..bounds.1 {
                                    t.push(function(j, i));
                                }
                            }

                            t
                        }));
                    }


                    temp.pixels = thread_vec
                        .into_iter()
                        .flat_map(|a| a.join().unwrap())
                        .collect::<Vec<_>>()
                });

                Ok(temp)
            }
        }
    }
}
