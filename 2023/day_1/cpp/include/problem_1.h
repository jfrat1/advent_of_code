#ifndef PROBLEM_1_H
#define PROBLEM_1_H

#include "file_reader.h"
#include "solver_base.h"

class ProblemOneSolver : protected SolverBase {
    public:
        ProblemOneSolver(const IFileReader* file_reader) : SolverBase(file_reader) { };
        int Solve();
};

#endif