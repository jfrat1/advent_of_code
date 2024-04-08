#include <numeric>

#include "problem_1.h"


int ProblemOneSolver::Solve() {
    std::vector<std::string> lines = this->ReadLines();

    std::vector<int> line_values {0};
    line_values.resize(lines.size());

    std::transform(
        lines.cbegin(),
        lines.cend(),
        line_values.begin(),
        [this](std::string line) {return this->ExtractLineValue(line);}
    );

    int sum_of_values = std::reduce(line_values.cbegin(), line_values.cend());
    return sum_of_values;
}
