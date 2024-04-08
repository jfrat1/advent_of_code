#include <gtest/gtest.h>
#include <gmock/gmock.h>

#include <vector>

#include "problem_1.h"


class MockFileReader : public IFileReader {
    public:
        MOCK_METHOD(FileLines, ReadLines, (), (const override));
};


class ProblemOneTest : ProblemOneSolver {
    public:
        ProblemOneTest(const IFileReader* file_reader) : ProblemOneSolver(file_reader) {};
        LineDigits<size_t> LineDigitPositions(std::string line) {
            return ProblemOneSolver::LineDigitPositions(line);
        }
        LineDigits<char> LineDigitChars(std::string line) {
            return ProblemOneSolver::LineDigitChars(line);
        }
        int ExtractLineValue(std::string line) {
            return ProblemOneSolver::ExtractLineValue(line);
        }
};

TEST(Problem1, LineDigitPositions) {
    MockFileReader mock_file_reader;
    ProblemOneTest problem_one = ProblemOneTest(&mock_file_reader);

    std::string line = "1one2";
    LineDigits<size_t> positions = problem_one.LineDigitPositions(line);
    LineDigits<size_t> expected = LineDigits<size_t> {0, 4};
    ASSERT_EQ(positions.first, expected.first);
    ASSERT_EQ(positions.last, expected.last);
}


TEST(Problem1, ExtractLineValueNominal) {
    MockFileReader mock_file_reader;
    ProblemOneTest problem_one = ProblemOneTest(&mock_file_reader);

    std::string line = "1one2";
    ASSERT_EQ(problem_one.ExtractLineValue(line), 12);
}

TEST(Problem1, ExtractLineValueSingleDigit) {
    MockFileReader mock_file_reader;
    ProblemOneTest problem_one = ProblemOneTest(&mock_file_reader);

    ASSERT_EQ(problem_one.ExtractLineValue("1two"), 11);
    ASSERT_EQ(problem_one.ExtractLineValue("two1"), 11);
}


TEST(Problem1, SolveSamplePuzzle) {
    MockFileReader mock_file_reader;
    std::vector<std::string> test_lines = std::vector {
        std::string("1abc2"),
        std::string("pqr3stu8vwx"),
        std::string("a1b2c3d4e5f"),
        std::string("treb7uchet"),
    };
    EXPECT_CALL(mock_file_reader, ReadLines())
        .WillOnce(testing::Return(test_lines));

    ProblemOneSolver problem_one = ProblemOneSolver(&mock_file_reader);
    int solution = problem_one.Solve();
    int expected = 142;

    ASSERT_EQ(solution, expected);
}

