#include <stdio.h>

#include "problem_1.h"


ProblemOne::ProblemOne(FILE* fp) {
    file_pointer = fp;
}

void ProblemOne::PrintLines() {
    const unsigned MAX_LENGTH = 256;
    char buffer[MAX_LENGTH];

     while (fgets(buffer, MAX_LENGTH, file_pointer))
        printf("%s", buffer);
}

int ProblemOne::Solve() {
    return 0;
}
