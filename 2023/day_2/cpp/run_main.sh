run_main() (
    echo "Running main\n"
    cd build/src && ./main && cd ../..
)

cmake -S . -B build \
&& cmake --build build \
&& run_main
