#include <gtest/gtest.h>

#include "solver_base.h"

class TestSolverBase : public SolverBase {
    public:
        TestSolverBase(const IFileReader* file_reader) : SolverBase(file_reader) { }

        int Solve() {
            return 0;
        }

        std::vector<std::string> ReadLines() {
            return SolverBase::ReadLines();
        }

};

TEST(TestSolverBase, ReadLines) {
    const char* filename = "data/file_reader_test_data.txt";
    FILE* fp = fopen(filename, "r");
    FileManager file_manager = FileManager(fp);
    FileReader file_reader = FileReader(&file_manager);


    TestSolverBase solver_base = TestSolverBase(&file_reader);
    std::vector<std::string> lines = solver_base.ReadLines();

    // This must match data from the file noted above
    std::vector<std::string> expected_lines {
        std::string("54nzzddht8ninelrkkseightseven6"),
        std::string("54nzzddht8ninelrkkseightseven6"),
        std::string("54nzzddht8ninelrkkseightseven6"),
        std::string("54nzzddht8ninelrkkseightseven6"),
    };

    ASSERT_EQ(lines, expected_lines);
}
