#ifndef PROBLEM_2_H
#define PROBLEM_2_H

#include <map>

#include "file_reader.h"
#include "solver_base.h"

typedef std::map<std::string, std::string> StringMap;
typedef std::pair<std::string, std::string> StringMapPair;

class ProblemTwoSolver : protected SolverBase {
    public:
        ProblemTwoSolver(const IFileReader* file_reader) : SolverBase(file_reader) { };
        int Solve();

    protected:
        std::string PreProcessLine(std::string line) const;
        // TODO - the overlapping spelling expansion and spelling->numeric replacement
        // are strcturally identical, but with a different replacement map as input.
        // Write a shared set of functions to perform this process.
        std::string ExpandAllOverlappingSpelledDigits(std::string line) const;
        std::string ExpandOverlappingSpelledDigit(std::string, StringMapPair overlapping_expansion_pair) const;
        std::string ConvertAllSpelledDigitsToNumbers(std::string line) const;
        std::string ConvertSpelledDigitToNumber(std::string line, StringMapPair spelled_to_numeric_pair) const;
        std::string ReplaceSubstring(
            std::string string,
            std::string substring,
            std::string replacement
        ) const;
        bool IsSubstringInString(std::string string, std::string substring) const;
        size_t SubstringStartPos(std::string string, std::string substring) const;

};

#endif