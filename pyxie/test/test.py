from pyxie import PyxiePictureGrey, grey
from random import randint

def col(x, y):
    c = x/200 + 1j*y/200 - 1 - 1j

    z = 0.32 + 0.5j

    for i in range(255):
        c = c**2 + z

        if abs(c) > 2:
            break

    return grey(255-i)

PyxiePictureGrey().from_fn(400, 400, col).save("test.png")