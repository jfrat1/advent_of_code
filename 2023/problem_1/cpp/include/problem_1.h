#ifndef PROBLEM_1_H
#define PROBLEM_1_H

#include <stdio.h>

class ProblemOne {
    public:
        int someNum;
        char* someChars;

        ProblemOne(FILE* fp);
        void PrintLines();
        int Solve();

    private:
        FILE* file_pointer;
};

#endif