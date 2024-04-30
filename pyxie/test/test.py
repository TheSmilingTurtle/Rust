from pyxie import PyxiePictureGrey, grey
from random import randint

def col(x: int, y: int, exp):
    c = x/150 + 1j*y/150 - 2 - 4j/3

    z = -0.8 + 0.156j

    for i in range(255):
        c = c**exp + z

        if abs(c) > 2:
            break
    return grey(255-i)

for i in range(8*30):
    exp = 2 + i/30
    print(f"Generating image {i}, exp is {exp}")
    PyxiePictureGrey().from_fn(600, 400, lambda x,y: col(x,y, exp)).save(f"/home/thesmilingturtle/coding/Rust/pyxie/movie/img{i}.png")