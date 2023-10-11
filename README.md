# The puzzle
Three possible actions are allowed: rotation, shift to the left, and shift to the right.  Using these actions, sort the numbers in the ascending order.
<img width="243" alt="Screenshot 2023-10-11 114914" src="https://github.com/bugaev/turnstyle/assets/1672453/b249aeca-1332-4497-94ef-9d1251186d1f">

# The program
The program explores trees with 3 child nodes in each node. Each child node corresponds to a chosen action (shifts or rotation). Both breadth-first and depth-first approaches were implemented, but only the latter provides rock-solid performance. Breadth-first approach is hungry for memory and realistically, cannot allocate enough memory for trees taller than ~20 nodes.

# Example output of the program
```
---------- Generation 1 -----------
---------- Generation 2 -----------
---------- Generation 3 -----------
---------- Generation 4 -----------
---------- Generation 5 -----------
---------- Generation 6 -----------
---------- Generation 7 -----------
---------- Generation 8 -----------
---------- Generation 9 -----------
---------- Generation 10 -----------
---------- Generation 11 -----------
---------- Generation 12 -----------
---------- Generation 13 -----------
Shift(Left)
Rotation
Shift(Left)
Shift(Left)
Rotation
Shift(Left)
Rotation
Shift(Right)
Rotation
Shift(Right)
Shift(Right)
Rotation
Shift(Right)
Nop
!!! Success !!!
End of the program.
```
