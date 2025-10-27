# Fitness

 A visualization tool written in [rust](https://www.rust-lang.org/)

 Produces a chart which shows the interval between bouts of activity.

 A markdown file with a similar format to

 ```markdown
 # Fitness
 12 Oct 2025
 | Sun   | Mon   | Tue   | Wed   | Thu   | Fri   | Sat   |
 |:-:    |:-:    |:-:    |:-:    |:-:    |:-:    |:-:    |
 |  X    |   X   | 17:00 | 10:00 |  X    | 04:00 | 09:00 |
 | 15:00 | 23:00 |   X   | 06:30 | 08:15 |       |       |
 ```

Which render to : -
 # Fitness
 12 Oct 2025
| Sun   | Mon   | Tue   | Wed   | Thu   | Fri   | Sat   |
| :-:   |:-:    |:-:    |:-:    |:-:    |:-:    |:-:    |
|   X   |   X   | 17:00 | 10:00 |  X    | 04:00 | 09:00 |
| 15:00 | 23:00 |   X   | 06:30 | 08:15 |       |       |



 ## How to use

 Pass a markdown file to  `StdIn`  and the program outputs a list of intervals(in hrs)

 eg

 24,
 25,
 30,

```bash
 #!/bin/bash
RUST_LOG=info cargo run < ./Fitness.md > interval.dat
gnuplot -p interval.gnuplot
```

 TODO