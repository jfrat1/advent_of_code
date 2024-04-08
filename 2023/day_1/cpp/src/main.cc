#include <stdio.h>

#include "file_reader.h"
#include "problem_1.h"
#include "problem_2.h"

int main() {
    const char *filename = "../data/problem_data.txt";
    FILE* fp = fopen(filename, "r");

    if (fp == NULL)
    {
        printf("Error: could not open file %s", filename);
        return 1;
    }

    FileManager file_manager = FileManager(fp);
    FileReader file_reader = FileReader(&file_manager);


    ProblemOneSolver problem_1 = ProblemOneSolver(&file_reader);
    int problem_1_solution = problem_1.Solve();
    printf("Problem 1 solution: %d\n", problem_1_solution);


    ProblemTwoSolver problem_2 = ProblemTwoSolver(&file_reader);
    int problem_2_solution = problem_2.Solve();
    printf("Problem 2 solution: %d\n", problem_2_solution);

    return 0;
}