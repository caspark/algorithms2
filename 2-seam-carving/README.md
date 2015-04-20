Seam Carving
============

Given an image, calculate the "energy" of each pixel according to the features on the image so that pixels with features
have higher energy, then use that information to remove vertical/horizontal paths of pixels with lowest energy until the
image is of the desired dimensions.

* Spec: http://coursera.cs.princeton.edu/algs4/assignments/seamCarving.html
* FAQ: http://coursera.cs.princeton.edu/algs4/checklists/seamCarving.html
* Sample inputs: http://coursera.cs.princeton.edu/algs4/testing/seamCarving-testing.zip

**Currently incomplete**

Sample execution:

```
cargo run -- seamCarving-testing/HJocean.png output.png
```
