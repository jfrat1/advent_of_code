#ifndef SOLVER_BASE_H
#define SOLVER_BASE_H

#include "file_reader.h"

// template <typename T>

// struct LineDigits {
//     T first;
//     T last;
// };



class SolverBase {
    public:
        virtual ~SolverBase() {};
        SolverBase(const IFileReader* file_reader);
        virtual int Solve() = 0;

    protected:
        const IFileReader* file_reader;

        std::vector<std::string> ReadLines() const;

        // int ExtractLineValue(std::string line) const;
};

#endif