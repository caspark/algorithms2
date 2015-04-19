WordNet
=======

Given a set of nouns, use breadth-first search over a directed graph of WordNet synsets to score each noun's distance
from all others, then use that score to find the outcast noun: the noun that's least related to each other noun.

* Spec: http://coursera.cs.princeton.edu/algs4/assignments/wordnet.html
* FAQ: http://coursera.cs.princeton.edu/algs4/checklists/wordnet.html
* Sample inputs: http://coursera.cs.princeton.edu/algs4/testing/wordnet-testing.zip

Sample execution:

```
cargo run --release -- wordnet-testing/synsets.txt wordnet-testing/hypernyms.txt wordnet-testing/outcast5.txt wordnet-testing/outcast8.txt wordnet-testing/outcast11.txt
```
