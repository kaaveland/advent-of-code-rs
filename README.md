Advent of Code solutions in Rust
==

My solutions for Advent of Code puzzles in Rust, created in 2022/2023. These have been solved
in reverse chronological starting in December 2022. I used these puzzles to learn to program Rust.

There's a small command line utility to run the programs and download data from advent of code. Most
of the programs are self-contained, there's very little shared code so that it should be relatively
easy to take out some code and run independently.

For year 2022, the focus was mostly to get familiar with Rust and the stdlib. There's a variety of
error handling, from none at all to matching against `Option`s or `Result`s to using the `?` operator
with `anyhow::Result`. For year 2021 I wanted to write code that performs well.

Usage
==

Build with: `cargo build --release` and run `target/release/aoc --help`:

```shell
$ target/release/aoc --help
Advent of Code toolset

Usage: aoc <COMMAND>

Commands:
  day-data  Get data for day (dump to input/year/day_nn/input
  data      Get data for all days
  run       Run solution, both parts, with timing
  runall    Run all known solutions, with individual and total timing
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

If you want to actually use this, you probably want to first run `target/release/aoc data` to
get a copy of your datasets locally. The output from `runall` looks something like this (these are my answers for 2021):

```shell
$ target/release/aoc runall 2021
Run all implemented solutions
Day 1 part 1: 104μs
1791
Day 1 part 2: 93μs
1822
Day 2 part 1: 89μs
2120749
Day 2 part 2: 55μs
2138382217
Day 3 part 1: 95μs
2954600
Day 3 part 2: 256μs
1662846
Day 4 part 1: 344μs
8136
Day 4 part 2: 483μs
12738
Day 5 part 1: 1148μs
5167
Day 5 part 2: 3ms
17604
Day 6 part 1: 14μs
379114
Day 6 part 2: 20μs
1702631502303
Day 7 part 1: 716μs
352254
Day 7 part 2: 1775μs
99053143
Day 8 part 1: 471μs
390
Day 8 part 2: 394μs
1011785
Day 9 part 1: 349μs
535
Day 9 part 2: 926μs
1122700
Day 10 part 1: 93μs
278475
Day 10 part 2: 136μs
3015539998
Day 11 part 1: 178μs
1649
Day 11 part 2: 270μs
256
Day 12 part 1: 153μs
4885
Day 12 part 2: 3ms
117095
Day 13 part 1: 323μs
735
Day 13 part 2: 364μs
#  # #### ###  #### #  #  ##  #  # ####
#  # #    #  #    # # #  #  # #  #    #
#  # ###  #  #   #  ##   #  # #  #   #
#  # #    ###   #   # #  #### #  #  #
#  # #    # #  #    # #  #  # #  # #
 ##  #    #  # #### #  # #  #  ##  ####

Day 14 part 1: 92μs
2975
Day 14 part 2: 137μs
3015383850689
Day 15 part 1: 784μs
696
Day 15 part 2: 23ms
2952
Day 16 part 1: 144μs
889
Day 16 part 2: 91μs
739303923668
Day 17 part 1: 797μs
7875
Day 17 part 2: 593μs
2321
Day 18 part 1: 360μs
3411
Day 18 part 2: 4ms
4680
Day 19 part 1: 11ms
419
Day 19 part 2: 10ms
13210
Day 20 part 1: 1870μs
4873
Day 20 part 2: 52ms
16394
Day 21 part 1: 8μs
707784
Day 21 part 2: 6ms
157595953724471
Day 22 part 1: 8ms
590467
Day 22 part 2: 3ms
1225064738333321
Day 23 part 1: 51ms
16059
Day 23 part 2: 47ms
43117
Day 24 part 1: 9ms
52926995971999
Day 24 part 2: 233μs
11811951311485
Day 25 part 1: 31ms
453
Day 25 part 2: 0μs
Submit the answers and click the button
All implemented solutions took: 100ms
```

This one was run in parallel on an AMD 5900X desktop CPU, but it's not much slower
run serially or on a laptop. The goal was to clock in under 1 second.

The solutions will run using [rayon](https://docs.rs/rayon/latest/rayon/) which is an excellent library
for data parallel computation. You can control the number of threads you want to use by setting an environment
variable for `RAYON_NUM_THREADS`, by default I suspect it uses all cores, including hyperthreads.

Code, structure and tests
==

[lib.rs](src/lib.rs) adds all the solution programs to a static data structure,
they are all public modules. [dl_data.rs](src/dl_data.rs) has some simple and
stupid code for connecting to adventofcode.com using a blocking [reqwest](https://docs.rs/reqwest/latest/reqwest/) 
http client by annoyingly prompting you for your cookie, which it does not store anywhere after
use. [aoc.rs](src/bin/aoc.rs) uses [clap](https://docs.rs/clap/latest/clap/) to
expose all this to the command line. The code uses [anyhow](https://docs.rs/anyhow/latest/anyhow/)
throughout to make the `?` operator a bit more ergonomic.

Most of the solution programs have tests; you can run them with `cargo test` or `cargo test --release`.

Learning points
==

Solution comments
--

2021 learning:

- [Day 19](src/year_2021/day_19.rs) was solved in one sitting on a train ride from Oslo to Trondheim and taught me
  a lot about thinking in 3D and cartesian coordinates. Obviously this has a much more elegant linear algebra
  solution, but I invented my technique from first principles and it was very rewarding to come up with it. It
  will only attempt rotations on 1 point when attempting to find out how to connect scanners, avoiding a lot
  of work at the cost of some complexity in finding out which point to rotate.
- [Day 22](src/year_2021/day_22.rs) has an interesting solution based on a tree of cuboid intersections where 
  volume alternates between being added and removed depending on the depth of the tree.
  It is fast and a _lot_ easier than attempting to split cubes. Draw some venn diagrams
  of a simplified version in 2D, and it should be easy to see why it works.
- [Day 23](src/year_2021/day_23.rs) was surprisingly easy to solve, once I got over myself and just started writing
  all the annoying rules. Then I got to part 2 and thought I was going to have to deal with changing my
  data types everywhere, but discovered that Rust has const generics. The implementation is a simple Dijkstra,
  and all the complexity is in managing state transitions. In hindsight, I think it may be simpler to just represent
  state as a bytestring here.
- [Day 5](src/year_2021/day_05.rs) has a much simpler solution than I wrote, I ended up solving equations more or less by hand to do this.
  It is much faster than using set intersection, but really hard to read and understand why it works. Not my proudest 
  moment, but it was also OK to do some simple math, I don't do a lot of this stuff for a living.
- [Day 20](src/year_2021/day_20.rs) I could probably revisit to optimize by changing the underlying datastructure from a `HashMap` to
  a `Vec`. I knew it was likely to be faster from the start, but the code is so much simpler when using a `HashMap` and I was
  in a mood to just get it done.
- [Day 12](src/year_2021/day_12.rs) is a simple depth first search, but I was very happy with how simple
  and fast the implementation ended up being.

Rust comments
--

- Cargo is super nice. I wish every language had such nice tooling. I got to play around with `cargo flamegraph`
  for some of these solutions, and I am just overall super-impressed. `clippy` is also a good way to learn faster.
- I really like keeping the tests and the code that is being tested so close.
- The standard library comes packed with efficient and easy-to use data structures and iterators.
- It still feels relatively hard to work with references once I need to mutate data. Sometimes I split up
  data that belongs together logically, in order to avoid needing both a mutable and an immutable reference
  into the same struct. This is counterintuitive. I wish the compiler would be a little smarter on this front.
- Performance is great.
- `Result<T, Error>` is great, and it never feels too rough to deal with the fact that basically everything can
  fail all the time, the `?` syntax helping a lot here.
- It is unfortunately a bit annoying to pass around iterators as arguments and the fact that there's no
  GC means that sometimes they must be consumed a lot sooner than I had planned for while writing the code.
  I'm still not really sure how to deal with the fact that each closure is a different type, I was often collecting
  data into `Vec`s that I would immediately consume again to work around this, or passing in mutable `Vec`s that I 
  could `extend()` iterators into. 
- Rust is relatively verbose. Often times I can find a clean and easy solution to a particular problem
  that I could _never_ manage to write in C, but then I have to deal with all the error conditions that I
  would never know about in C afterwards. Overall it doesn't feel too verbose, but I would have less than
  half of the code in some other languages I know.
- It is easy, fun and efficient to write simple CLI software in Rust. Compared to C, it's much faster to do so,
  both because the language aligns more with how I think and the stdlib has batteries included, but also all kinds
  of things are just a `cargo add` away, f. ex. easy-to-use HTTP client.
- On the whole I think I'd like to work professionally with Rust at some point, it's really fun.