# quaterdrop
_or_
# Mandelbrot: s/ℂ/ℍ Edition

Named thusly 'cuz it kind of looks like a water drop at the top level. Also quaternions.

This is a GPU program that draws the mandelbrot set, using quaternions instead of complex numbers.
(Quaternion fractals aren't a new idea, except to me.)

You probably want a reasonably good GPU.

Something approximately equivalent to the Julia Set available by editing `main` in `fragment.glsl`.
It's somewhat more difficult to find interesting space in it; an iteration depth of 200 is recommended.


# Controls
```
left-drag: pan x,y
right-drag: pan z,w
scroll: zoom (x,y)
r: reload shader.
o/p: change iteration depth limit by 1
lbracket/rbracket: change iteration depth limit by 10.
space: reset zoom
period: print draw parameters
```

# Interesting features
Collect all __n__.

* Maelstorms
* Theatres
* Spooky ghosts
* Creepy masks




The fractal viewer bit is forked from [mandelbrot-rust-gl](https://github.com/remexre/mandelbrot-rust-gl).
