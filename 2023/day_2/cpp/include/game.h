#include <stdint.h>
#include <string>
#include <vector>

typedef uint16_t RedCubes;
typedef uint16_t GreenCubes;
typedef uint16_t BlueCubes;


struct Cubes {
    RedCubes red;
    GreenCubes green;
    BlueCubes blue;
};

class Reveal {
    public:
        Reveal(Cubes revealed_cubes);

        static Reveal From(std::string reveal_str);

        bool IsRevealPossible(Cubes available_cubes);
        Cubes GetCubes();

    protected:
        Cubes revealed_cubes;
};

#define EXPECTED_NUM_REVEALS_PER_GAME 3

class Game {
    public:
        Game();

        static Game From(std::string line);

        void AddReveal(Reveal reveal);
        std::vector<Reveal> Reveals();
        int NumReveals();
        bool IsGamePossible(Cubes available_cubes);

    protected:
        std::vector<Reveal> reveals;

};
