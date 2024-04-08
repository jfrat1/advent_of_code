enable_testing()

set(SOURCE_DATA_FILE_PATH  "${PROJECT_SOURCE_DIR}/tests/data/file_reader_test_data.txt")
set(BUILD_DATA_FOLDER_PATH  "${PROJECT_SOURCE_DIR}/build/tests/data")
file(COPY "${SOURCE_DATA_FILE_PATH}" DESTINATION "${BUILD_DATA_FOLDER_PATH}")

add_executable(
    file_reader_test
    file_reader_test.cc
)
target_link_libraries(
    file_reader_test
    file_reader
    GTest::gtest_main
    GTest::gmock_main
)

add_executable(
    problem_1_test
    problem_1_test.cc
)
target_link_libraries(
    problem_1_test
    problem_1
    GTest::gtest_main
    GTest::gmock_main
)

add_executable(
    problem_2_test
    problem_2_test.cc
)
target_link_libraries(
    problem_2_test
    problem_2
    GTest::gtest_main
    GTest::gmock_main
)


include(GoogleTest)
gtest_discover_tests(file_reader_test)
gtest_discover_tests(problem_1_test)
gtest_discover_tests(problem_2_test)