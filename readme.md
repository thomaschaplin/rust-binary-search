# rust-binary-search

Binary search algorithm written in [rust](https://www.rust-lang.org/)

## Description

In computer science, binary search, also known as half-interval search, logarithmic search, or binary chop, is a search algorithm that finds the position of a target value within a sorted array. Binary search compares the target value to the middle element of the array.

* Worst complexity: O(log n)
* Average complexity: O(log n)
* Best complexity: O(1)
* Space complexity: O(1)
* Data structure: Array
* Class: Search algorithm

## Setup

Make sure you have [rust](https://www.rust-lang.org/) installed on your machine by following the [getting started guide](https://www.rust-lang.org/learn/get-started)

## Instructions

* Clone this repository `git clone git@github.com:thomaschaplin/rust-binary-search.git`
* Change directory `cd rust-binary-search`
* Build the application `cargo build`
* Run the application `cargo run`
* Test the application `cargo test`

#### Example input of run:

```rust
let arr: [i32; 20] = [
    1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155, 162, 176, 188, 199, 200, 210, 222,
];
let target: i32 = 88;
```

#### Example output of run:

```
Found 88 at index 7 of the array.
```

#### Example output of test:

```
running 2 tests
test tests::assert_value_is_found ... ok
test tests::assert_value_is_not_found ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# Docker Setup

Build
```
docker build --rm -f Dockerfile -t thomaschaplin:rust-binary-search .
```

Run
```
docker run --rm -it thomaschaplin:rust-binary-search
```