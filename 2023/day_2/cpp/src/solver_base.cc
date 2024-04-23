
#include "solver_base.h"

SolverBase::SolverBase(const IFileReader* file_reader) {
    this->file_reader = file_reader;
}

std::vector<std::string> SolverBase::ReadLines() const {
    return this->file_reader->ReadLines();
}
