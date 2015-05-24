Burrows-Wheeler Transform
=========================

Implement part of the Burrows-Wheeler data compression algorithm by implementing move-to-front encoding (and decoding), a circular suffix array, and finally the actual transform. (The final step, Huffman encoding and decoding, is provided [as Java code](http://algs4.cs.princeton.edu/55compression/Huffman.java.html), and currently isn't ported to Rust to get the actual full-blown data compression algorithm.)

* Spec: http://coursera.cs.princeton.edu/algs4/assignments/burrows.html
* FAQ: http://coursera.cs.princeton.edu/algs4/checklists/burrows.html
* Sample inputs: http://coursera.cs.princeton.edu/algs4/testing/burrows-testing.zip

Sample execution:

```
cargo build && \
    echo -n "Burrows-Wheeler transform and move-to-front transform!" | \
    ./target/debug/6-burrows-wheeler burrows-wheeler encode | \
    ./target/debug/6-burrows-wheeler move-to-front encode | \
    ./target/debug/6-burrows-wheeler move-to-front decode | \
    ./target/debug/6-burrows-wheeler burrows-wheeler decode | \
    xxd
```
