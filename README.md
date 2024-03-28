# roll
A command line dice roller to learn Rust.

Currently supported:

`roll` defaults to 1d20

`roll d8` rolls 1d8

`roll 4d20` rolls d20 4 times

`roll 4d20 -v` will show verbose: the result of each roll and the sum of all rolls

`roll 4d20 -d` will show how each dice was instantiated


Plans include supporting things like:

```
roll d20 + 4
24

roll 2d6, 2d8
38

roll 2d20 --distinct
20 20

roll 2d20 --distinct --advantage
20 16

roll 2d20 --distinct --disadvantage
16 20

roll 2d20 --distinct --separator=, --enclosure=[]
[20, 20]

