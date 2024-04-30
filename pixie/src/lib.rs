use num::Unsigned;
use image::{ImageBuffer, Pixel, Luma, Rgb, EncodableLayout, PixelWithColorType};
use rayon::prelude::*;

pub mod prelude{
    pub use super::{grey, rgb};
    pub use super::Picture;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn flow(){
        Picture::from_fn(
                    6001, 
                    4001, 
                    |x, y| {
                        if ((x as i64)-3000).pow(2) + ((y as i64)-2000).pow(2) < (1200 as u32).pow(2 as u32) as i64 {
                            rgb(238, 0, 0)
                        } else {
                            rgb(255, 255, 255)
                        }
                    }
                ).save("test.png");

        ()
    }

    #[test]
    fn flow_par(){
        Picture::scoped_viki_par(
                    6001, 
                    4001, 
                    &|x, y| {
                        if ((x as i64)-3000).pow(2) + ((y as i64)-2000).pow(2) < (1200 as u32).pow(2 as u32) as i64 {
                            rgb(238, 0, 0)
                        } else {
                            rgb(255, 255, 255)
                        }
                    },
                    4
        ).save("test.png");

        ()
    }
}

pub fn grey(val: u8) -> Luma<u8> {
    Luma([val])
}

pub fn grey8(val: u8) -> Luma<u8> {
    Luma([val])
}

pub fn grey16(val: u16) -> Luma<u16> {
    Luma([val])
}

pub fn rgb(r: u8, g: u8, b: u8) -> Rgb<u8> {
    Rgb([r, g, b])
}

pub fn rgb8(r: u8, g: u8, b: u8) -> Rgb<u8> {
    Rgb([r, g, b])
}

pub fn rgb16(r: u16, g: u16, b: u16) -> Rgb<u16> {
    Rgb([r, g, b])
}

pub struct Picture<P: Pixel, U: Unsigned>{
    image: ImageBuffer<P, Vec<U>>
}

impl<P: Pixel<Subpixel = U>, U: Unsigned> Picture<P, U>{
    pub fn from_fn<F>(w: u32, h: u32, func: F) -> Self 
    where F: Fn(u32, u32) -> P
    {
        Picture{ image: ImageBuffer::from_fn(
                                w, 
                                h, 
                                func
                                )
                }
    }
}

impl<P, U: Unsigned + Send + Copy> Picture<P, U>
where P: Pixel<Subpixel = U> + Send,
{
    pub fn rayon_par(w: u32, h: u32, f: &(dyn Fn(u32, u32) -> P + Send + Sync)) -> Self
    {
        let temp: Vec<U> = (0..(w*h)).into_par_iter().map(|i| f(i%w, i/w).channels().to_vec()).flatten().collect();

        Picture{ 
            image: ImageBuffer::from_raw(w, h, temp).unwrap()
        }
    }
}

impl<P, U: Unsigned + Send + Copy + std::fmt::Debug> Picture<P, U>
where P: Pixel<Subpixel = U> + Send,
{
    pub fn viki_par(w: u32, h: u32, f: &(dyn Fn(u32, u32) -> P + Send + Sync), threads: usize) -> Self
    {
        let chunk_size = ((w*h) as usize)/threads as usize + 1;

        let mut pixels: Vec<Vec<U>> = vec![vec![]; (w*h) as usize];

        crossbeam::scope(|spawner| {
            for (i, chunk) in pixels.chunks_mut(chunk_size).into_iter().enumerate() {

                spawner.spawn(move |_| {
                    for (j, e) in chunk.iter_mut().enumerate(){
                        *e = f( ( (i*chunk_size + j) as u32 )%w, ( (i*chunk_size + j) as u32 )/w ).channels().to_vec();
                    }
                });

            }

        }).unwrap();

        Picture{
            image: ImageBuffer::from_raw(w, h, pixels.into_iter().flatten().collect::<Vec<U>>()).unwrap()
        }
    }
}

impl<P, U: Unsigned + Send + Copy> Picture<P, U>
where P: Pixel<Subpixel = U> + Send,
{
    pub fn from_fn_par(w: u32, h: u32, f: &'static (dyn Fn(u32, u32) -> P + Send + Sync), threads: u32) -> Self
    {
        let thread_count_upper = (w*h) - threads * ( (w*h) / threads );
        let chunk_length_lower = ( (w*h)/threads).clamp(1, w*h);
        let chunk_length_upper = chunk_length_lower + 1;

        let temp: Vec<Vec<U>> = std::thread::scope(|s| {
            let mut thread_vec= Vec::with_capacity((threads+1) as usize);

            for start in 0..thread_count_upper {
                thread_vec.push(s.spawn(move ||{
                    let mut tmp = Vec::with_capacity(chunk_length_upper as usize);

                    for i in 0..chunk_length_upper{
                        tmp.push(f((start*chunk_length_upper + i)%w, (start*chunk_length_upper + i)/w).channels().to_vec());
                    }

                    tmp
                }))
            }

            for start in 0..(threads-thread_count_upper) {
                thread_vec.push(s.spawn(move ||{
                    let mut tmp = Vec::with_capacity(chunk_length_lower as usize);

                    for i in 0..chunk_length_lower{
                        tmp.push(f((thread_count_upper*chunk_length_upper + start*chunk_length_lower + i)%w, (thread_count_upper*chunk_length_upper + start*chunk_length_lower + i)/w).channels().to_vec());
                    }

                    tmp
                }))
            }

            thread_vec
                .into_iter()
                .flat_map(|a| a.join().unwrap())
                .collect()
        });
        
        Picture{ 
            image: ImageBuffer::from_raw(w, h, temp.into_iter().flatten().collect()).unwrap()
        }
    }
}

impl<P: PixelWithColorType + Pixel<Subpixel = U>, U: Unsigned> Picture<P, U>
where [U]: EncodableLayout{
    pub fn save(&self, path: &str) {
        let _ = self.image.save(path);
    }
}

impl<P, U: Unsigned + Send +  Clone> Picture<P, U>
where P: Pixel<Subpixel = U>,
{
    pub fn scoped_viki_par(w: u32, h: u32, f: &'static (dyn Fn(u32, u32) -> P + Sync), threads: u32) -> Self
    {
        let chunk_length = ((w*h)/threads + 1).clamp(1, w*h);

        let mut temp: Vec<Vec<U>> = vec![vec![]; (w*h) as usize];
        
        std::thread::scope(|s| {
            let mut thread_vec= Vec::with_capacity(threads as usize);

            for (i, chunk) in (&mut temp).chunks_mut(chunk_length as usize).into_iter().enumerate(){
                thread_vec.push(s.spawn(move ||{
                    for (j, e) in chunk.iter_mut().enumerate(){
                        *e = f(((i as u32)*chunk_length + j as u32)%w, ((i as u32)*chunk_length + j as u32)/w).channels().to_vec();
                    }
                }));

            }

        });
        
        Picture{ 
            image: ImageBuffer::from_raw(w, h, temp.into_iter().flatten().collect()).unwrap()
        }
    }
}
