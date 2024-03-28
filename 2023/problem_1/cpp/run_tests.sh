run_tests() (
    cd build/tests \
    && ctest --output-on-failure \
    || cd -
)

cmake -S . -B build \
&& cmake --build build \
&& run_tests