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

```
OPTIONS
    -h, --help - Show this help page
    -g, --generate <NUMBER> - Generates random cities in a specified file.\
    If no directory is specified, it will generate the cities in ./data
    -o, --output <STRING> - Specify output file.
    -i, --input <STRING> - Specify input file. If no file is specified, it \
    reads from ./data
GENETIC PROCESS
    -p, --population <NUMBER> - Specify the population (Default is 5000).
    -m, --mutation <FLOAT> - Specify the mutation rate (Default is 0.5).
    -t, --truncation <NUMBER> - Specify the truncation limit (Default if 30). This is % of the population that will be selected\
    to populate the next generation
UI CONTROLS
    <SPACE> - Pause and unpause
    <ESC> - Terminate the program
    s - Toggle explorative and exploitative [s]election.
    r - Toggle explorative and exploitative [r]epopulation.
    i - [I]terate. Iterate by one step. Only works while paused.
```


First generate the cities with the -g `number of cities` command. The output file can be specified with -o

Then run the genetic algorithm. You can specify the input file with -i

If you wish, you can custom set the parameters. Here is an example:

`genetic_algorithm -p 10000 -m 0.4 -t 20`

Which sets the population to 10000, mutation rate to 0.4 and truncation to 20% respectively.


