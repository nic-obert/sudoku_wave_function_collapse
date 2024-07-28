# Wave function collapse Sudoku solver

A Sudoku board solver and generator written in Rust.


- [Wave function collapse Sudoku solver](#wave-function-collapse-sudoku-solver)
  - [Basic usage](#basic-usage)
  - [Basic concepts](#basic-concepts)
    - [Wave function collapse](#wave-function-collapse)
    - [Wave functions in Sudoku](#wave-functions-in-sudoku)
- [How to generate a Sudoku board](#how-to-generate-a-sudoku-board)
  - [Procedural wave function collapse](#procedural-wave-function-collapse)
  - [Procedural wave function collapse with uniqueness checks](#procedural-wave-function-collapse-with-uniqueness-checks)
- [How to solve a Sudoku board](#how-to-solve-a-sudoku-board)
  - [Wave function collapse with brute-force backtracking](#wave-function-collapse-with-brute-force-backtracking)
  - [Neighboring wave analysis with backtracking](#neighboring-wave-analysis-with-backtracking)
- [License](#license)


## Basic usage 

## Basic concepts

Before we get to the algorithms, let's get familiar with the related concepts and data structures. 

### Wave function collapse

The wave function collapse algorithm is a technique inspired by quantum mechanics. In simple words, very small particles normally behave like probability waves. However, when the probability wave is observed, it collapses into a determinate particle.  
Furthermore, consider a pair of entangled particles. If the spin of one of them is measured, the spin of the other particle is automatically determined to be the opposite of its twin's. This means that the state of one particle is constrained by the state of the other.

Wave function collapse algorithms borrow from the concepts of quantum constraints and probability wave collapse by assigning a wave function to certain items. These wave functions are then procedurally collapsed into determinate states.  
Whenever an item's wave function collapses, the wave functions of entangled items are affected based on the constraints placed by the newly collapsed state. If, as a result of the previous collapse, an entangled wave function happens to have only one possible state, it collapses in turn, affecting other entangled wave functions in a chain reaction.

### Wave functions in Sudoku

In the case of Sudoku, the range of possible states a certain cell can assume is represented by a wave function. Such a wave function is dependent on the constraints put in place by the neighboring cells according to the rules of Sudoku.


# How to generate a Sudoku board

There are many techniques to generate a Sudoku board. This respository implements some basic techniques to generate a random valid board.

## Procedural wave function collapse

This technique generates a valid pseudo-random Sudoku board with at least one possible solution. The solution is not guaranteed to be unique, though. To generate boards with unique solutions, check out the next chapter.

First, a new board is generated with maximum entropy, meaning all cells can assume every possible state. 
We can then iterate through every cell of the board 

## Procedural wave function collapse with uniqueness checks


# How to solve a Sudoku board

Solving a Sudoku board can be a complicated task, especially if the given hints are few. This repository implements some techniques to solve valid Sudoku boards using algorithms based on wave function collapse.

## Wave function collapse with brute-force backtracking



## Neighboring wave analysis with backtracking 


# License

This repository and all its contents are licensed under the [MIT license](LICENSE).

