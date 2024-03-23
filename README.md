# roll
A command line dice roller to learn Rust.

Usage variations I'd like this to support, in order of potential complexity (tbd). Coverage will be conventional dice, with dn via override, n <= 1000:

```
roll
20

roll d20
20

roll d20 + 4
24

roll 2d20
40

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

roll d20 --verbose
You rolled a d20 and got a 20.

roll 2d20 --verbose --template="A [result] from a [dice]!"
A 40 from a 2d20!

roll 3d20 --verbose --distinct --template="A [result] from a [dice]!"
A 12 from a d20!
A 18 from a d20!
A 20 from a d20!

roll d777 --allow_unconventional
777