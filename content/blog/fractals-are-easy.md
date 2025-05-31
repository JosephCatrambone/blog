+++
title = "Fractals are Easy!"
date = "2013-04-29"

[taxonomies]
tags=["Programming"]
+++

I never realized how trivial it was to implement the Mandelbrot Set in Python. It's only about 25 lines.[![Mandelbrot](http://www.josephcatrambone.com/wp-content/uploads/2013/04/mandelbrot.png)](./img/wp-content-uploads-2013-04-mandelbrot.png)Here's the full code:

```
#!/usr/bin/env python
from __future__ import division
import Image

def make_mandelbrot(width, height, max_depth=255):
	i = Image.new('L', (width, height));
	for y in range(0, height):
		for x in range(0, width):
			real_component = 2*(x/width) + -2*((width-x)/width);
			imaginary_component = 2*(y/height) + -2*((height-y)/height);
			c_value = complex(real_component, imaginary_component);
			z_value = complex(0, 0);
			iteration_count = 0;
			while abs(z_value) < 2 and iteration_count < max_depth:
				z_value = z_value*z_value + c_value;
				iteration_count += 1;
			i.putpixel((x,y), 255*iteration_count/max_depth)
	i.show();
	return i;

if __name__=="__main__":
	img = make_mandelbrot(500,500);
	img.save('output.png');
```
