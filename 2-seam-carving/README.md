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
cargo run --release -- seamCarving-testing/HJocean.png -o /tmp/output.png -W 200 && pinta /tmp/output.png &> /dev/null
```

Also try `-e` to show calculated energy and `-p` to show the preview of the next seam that would be removed.
