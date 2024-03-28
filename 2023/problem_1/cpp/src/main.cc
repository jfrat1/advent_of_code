#include <stdio.h>

#include "problem_1.h"

int main() {
    const char *filename = "../data/problem_data.txt";
    FILE* fp = fopen(filename, "r");

    if (fp == NULL)
    {
        printf("Error: could not open file %s", filename);
        return 1;
    }

    ProblemOne problem_1 = ProblemOne(fp);
    problem_1.PrintLines();
    printf("Between printlines calls");
    rewind(fp); // important to run this between any usages of a single file pointer

    problem_1.PrintLines();


    fclose(fp);

    return 0;
}