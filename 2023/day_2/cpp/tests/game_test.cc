#include <gtest/gtest.h>

#include "game.h"


bool operator==(Reveal left, Reveal right) {
    Cubes left_cubes = left.GetCubes();
    Cubes right_cubes = right.GetCubes();

    return (
        left_cubes.red == right_cubes.red
        && left_cubes.green == right_cubes.green
        && left_cubes.blue == right_cubes.blue
    );
}

bool operator==(Game left, Game right) {
    std::vector<Reveal> left_reveals = left.Reveals();
    std::vector<Reveal> right_reveals = right.Reveals();

    if (left_reveals.size() != right_reveals.size()) {
        return false;
    }

    size_t size = left_reveals.size();
    for (int i = 0; i < size; i++) {
        if (left_reveals.at(i) != right_reveals.at(i)) {
            return false;
        }
    }

    return true;
}

TEST(Reveal, Construct) {
    Reveal reveal = Reveal(
        Cubes {RedCubes {3}, GreenCubes {4}, BlueCubes{ 0}}
    );
}

TEST(Reveal, FromStr) {
    std::string reveal_str = "12 red, 2 green, 5 blue";
    Reveal reveal = Reveal::From(reveal_str);

    Reveal expected_reveal = Reveal(
        Cubes{
            .red = 12,
            .green = 2,
            .blue = 5,
        }
    );
    ASSERT_EQ(reveal, expected_reveal);
}

TEST(Reveal, IsRevealPossibleTrue) {
    Reveal reveal = Reveal(
        Cubes {RedCubes {3}, GreenCubes {4}, BlueCubes {0}}
    );

    Cubes available_cubes = Cubes {RedCubes {5}, GreenCubes {6}, BlueCubes {2}};
    ASSERT_TRUE(reveal.IsRevealPossible(available_cubes));
}

TEST(Reveal, IsRevealPossibleFalse) {
    Reveal reveal = Reveal(
        Cubes {RedCubes {3}, GreenCubes {4}, BlueCubes {0}}
    );

    Cubes available_cubes = Cubes {RedCubes {2}, GreenCubes {6}, BlueCubes {2}};
    ASSERT_FALSE(reveal.IsRevealPossible(available_cubes));
}

TEST(Reveal, IsRevealPossibleTrueWithExactCubeMatch) {
    Reveal reveal = Reveal(
        Cubes {RedCubes {3}, GreenCubes {4}, BlueCubes {0}}
    );

    Cubes available_cubes = Cubes {RedCubes {3}, GreenCubes {4}, BlueCubes {0}};
    ASSERT_TRUE(reveal.IsRevealPossible(available_cubes));
}

TEST(Game, ConstructEmpty) {
    Game game = Game();
}

TEST(Game, AddReveal) {
    Game game = Game();
    Reveal reveal = Reveal(
        Cubes {RedCubes {3}, GreenCubes {4}, BlueCubes {0}}
    );
    game.AddReveal(reveal);
}

TEST(Game, AddAndGetReveal) {
    Game game = Game();
    Reveal reveal = Reveal(
        Cubes {RedCubes {3}, GreenCubes {4}, BlueCubes {0}}
    );
    game.AddReveal(reveal);

    std::vector<Reveal> reveals = game.Reveals();
    ASSERT_EQ(reveals.size(), 1);
    ASSERT_EQ(reveals.at(0), reveal);
}

TEST(Game, AddMoreThanThreeReveals) {
    Game game = Game();
    Reveal reveal = Reveal(
        Cubes {RedCubes {3}, GreenCubes {4}, BlueCubes {0}}
    );
    game.AddReveal(reveal);
    game.AddReveal(reveal);
    game.AddReveal(reveal);
    game.AddReveal(reveal);

    std::vector<Reveal> reveals = game.Reveals();
    ASSERT_EQ(reveals.size(), 4);
    ASSERT_EQ(reveals.at(0), reveal);
}

TEST(Game, IsGamePossibleTrue) {
    Game game = Game();
    game.AddReveal(Reveal(Cubes{3, 6, 4}));
    game.AddReveal(Reveal(Cubes{5, 0, 12}));
    game.AddReveal(Reveal(Cubes{0, 15, 6}));

    Cubes available_cubes = Cubes{6, 16, 12};

    ASSERT_TRUE(game.IsGamePossible(available_cubes));
}

TEST(Game, IsGamePossibleFalse) {
    Game game = Game();
    game.AddReveal(Reveal(Cubes{3, 6, 4}));
    game.AddReveal(Reveal(Cubes{5, 0, 12}));
    game.AddReveal(Reveal(Cubes{0, 15, 6}));

    Cubes available_cubes = Cubes{6, 14, 10};

    ASSERT_FALSE(game.IsGamePossible(available_cubes));
}

TEST(Game, FromLine) {
    std::string line = "Game 1: 12 red, 2 green, 5 blue; 9 red, 6 green, 4 blue; 10 red, 2 green, 5 blue; 8 blue, 9 red";
    Game game = Game::From(line);

    Game expected_game = Game();
    expected_game.AddReveal(Reveal{Cubes{RedCubes{12}, GreenCubes{2}, BlueCubes{5}}});
    expected_game.AddReveal(Reveal{Cubes{RedCubes{9}, GreenCubes{6}, BlueCubes{4}}});
    expected_game.AddReveal(Reveal{Cubes{RedCubes{9}, GreenCubes{0}, BlueCubes{8}}});

    ASSERT_EQ(game, expected_game);
}