# Collatz Fractal Generator

![Collatz Fractal](https://github.com/thyrgle/collatz_fractal/blob/master/src/fractal.png)

<hr>

Among the many fractals, there is [Collatz Fractal](http://yozh.org/2012/01/12/the_collatz_fractal/) based on a complex extension of: 

```
def f(x):
    if x % 2 == 0:
        return x / 2
    return 3 * x + 1
```

To generate the fractal, you pick a bunch of points and repeatedly apply `f` over and over again a large number of times. Morally, `f(f(f(f(....f(x)....))))`. In this case, however, it is not a real number x but instead a complex number (often denoted z). The end result is plotted by giving it a color that "corresponds" to the resulting magnitude.

<hr>

Heavily modified variant of the Mandelbrot example [given here](https://github.com/PistonDevelopers/image).
