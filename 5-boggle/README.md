Boggle
======

Given a dictionary and a Boggle board, calculate the maximum score possible from it.

* Spec: http://coursera.cs.princeton.edu/algs4/assignments/boggle.html
* FAQ: http://coursera.cs.princeton.edu/algs4/checklists/boggle.html
* Sample inputs: http://coursera.cs.princeton.edu/algs4/testing/boggle-testing.zip

Status: complete, however it's not clear whether Q without a U after it on the board should be treated as QU anyway. I've elected to do so because the assignment says:

> The Qu special case. In the English language, the letter Q is almost always followed by the letter U. Consequently, the side of one die is printed with the two-letter sequence Qu instead of Q (and this two-letter sequence must be used together when forming words).
> ...
> You can assume the integers are nonnegative and that the characters are uppercase letters A through Z (with the two-letter sequence Qu represented as either Q or Qu).

Specifically, that last line indicates that Q on its own should be treated as QU. However, this does make it impossible to find the word QWERTY on the `board-qwerty.txt` input file; perhaps that's the expected behaviour.

Sample executions:

```
cargo run --release -- boggle-testing/dictionary-yawl.txt boggle-testing/board-points4540.txt
cargo run --release -- boggle-testing/dictionary-yawl.txt boggle-testing/board-points26539.txt
cargo run --release -- boggle-testing/dictionary-yawl.txt boggle-testing/board-pneumonoultramicroscopicsilicovolcanoconiosis.txt
cargo run --release -- boggle-testing/dictionary-yawl.txt boggle-testing/board-quinquevalencies.txt
cargo run --release -- boggle-testing/dictionary-algs4.txt boggle-testing/board-vertical.txt
```

