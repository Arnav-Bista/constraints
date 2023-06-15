# genetic_algorithms

## What is it?

This is a Genetic Algorithm written in Rust to solve the Travelling Salesman Problem.

This project was inspired by my IB CS final year case study about how we use Genetic Algorithms to solve problems that can't be solved with orthodox approaches without being computationally inefficient - especially the TSP. We were taught and had to research the theory behind genetic algorithms, what they do, how they work and how the parameters affect the solution; however, we were not expected to code the actual algorithm up.

I found it intriguing how we simulated nature's evolutionary process to solve a computationally expensive task and decided to try and solve this problem myself. 

This is the result of multiple trials and errors. The initial program was written in Julia, rewritten in Java and finally in Rust as a complete algorithm. 

## How does it work?

To be a bit technical, a genetic algorithm is a method for solving both constrained and unconstrained optimization problems based on a natural selection process that mimics biological evolution. It can be used in problems where the number of possible solutions increases rapidly and where primitive solutions such as brute force methods might be computationally infeasible.

The selection of the fittest individuals from a population begins the natural selection process. They generate offspring who inherit the parents' qualities and are passed down to the next generation. If parents have high fitness, their children will be fitter than they are and have a better chance of surviving.

 This process is known as natural selection and this can be used to find the solution for a search problem. We consider the best solutions and select the set of the fittest solutions.

These offsprings have a chance to undergo mutation where some chromosomes in their DNA are swapped. It helps promote biodiversity and helps the algorithm take a break from exploitation to perform exploration. 

This process is then repeated for a certain number of generations or until an end criterion is met. 

## How do I use it myself?

Although the genetic algorithm itself is complete, polishing and visualization are the only remaining aspects left. As soon as they are done, this will be updated to show instructions on how to run the program as well as contain example runs.
