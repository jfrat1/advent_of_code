#include <algorithm>
#include <optional>
#include <stdio.h>
#include "game.h"


Reveal::Reveal(Cubes revealed_cubes) {
    this->revealed_cubes = revealed_cubes;
}

Reveal Reveal::From(std::string reveal_str) {
    std::optional<RedCubes> red = std::nullopt;
    std::optional<BlueCubes> blue = std::nullopt;
    std::optional<GreenCubes> green = std::nullopt;

    size_t token_start_pos = 0;
    size_t token_end_pos = reveal_str.find(',');

    while (token_start_pos != std::string::npos) {
        size_t token_len = token_end_pos - token_start_pos;
        std::string token = reveal_str.substr(token_start_pos, token_len);
        // TODO: start here when I get back to this problem
    }


    return Reveal(Cubes{0, 0, 0});
}

bool Reveal::IsRevealPossible(Cubes available_cubes) {
    bool is_red_possible = available_cubes.red >= this->revealed_cubes.red;
    bool is_green_possible = available_cubes.green >= this->revealed_cubes.green;
    bool is_blue_possible = available_cubes.blue >= this->revealed_cubes.blue;

    return is_red_possible && is_green_possible && is_blue_possible;
}

Cubes Reveal::GetCubes() {
    return this->revealed_cubes;
}

#define EXPECTED_NUM_REVEALS_PER_GAME 3

Game::Game() {
    this->reveals.reserve(EXPECTED_NUM_REVEALS_PER_GAME);
}

#define GAME_ID_DELIMITER ':'
#define GAME_REVEALS_DELIMITER ';'
#define REVEAL_CUBES_DELIMITER ','

Game Game::From(std::string line) {
    size_t game_id_token_delim = line.find(':');
    std::string game_id_token = line.substr(0, game_id_token_delim);
    printf("\n%s\n", game_id_token.c_str());

    line.erase(0, game_id_token_delim+1);

    std::vector<std::string> cube_reveal_tokens (EXPECTED_NUM_REVEALS_PER_GAME);
    size_t pos = 0;
    while (pos < line.size()) {
        // TODO: this could be much cleaner and easier to understand
        pos = line.find(';');
        std::string this_game_cube_token = line.substr(0, pos);
        cube_reveal_tokens.push_back(this_game_cube_token);
        printf("Found game cubes: %s\n", this_game_cube_token.c_str());

        line.erase(0, pos+1);
    }

    for (size_t i = 0; i < cube_reveal_tokens.size(); i++) {
        std::string reveal_token = cube_reveal_tokens.at(i);
        size_t pos = 0;
        // TODO: make this magic '3' a number of cube colors
        std::vector<std::string> cube_tokens (3);

        // TODO: this loop is even more difficult to understand than the one above.
        while (pos < reveal_token.size()) {
            pos = reveal_token.find(',');
            if (pos == std::string::npos) {
                cube_tokens.push_back(reveal_token);
                printf("Found Cube: %s\n", reveal_token.c_str());
                reveal_token.erase(0, reveal_token.size());
            } else {
                std::string this_cube_token = reveal_token.substr(0, pos);
                cube_tokens.push_back(this_cube_token);
                printf("Found Cube: %s\n", this_cube_token.c_str());
                reveal_token.erase(0, pos + 1);
            }
            pos = 1;
        }
    }


    return Game();
}

void Game::AddReveal(Reveal reveal) {
    this->reveals.push_back(reveal);
}

std::vector<Reveal> Game::Reveals() {
    return this->reveals;
}

int Game::NumReveals() {
    return this->reveals.size();
}

bool Game::IsGamePossible(Cubes available_cubes) {
    std::vector<bool> is_game_possible (this->reveals.size());
    bool are_all_reveals_possible = std::all_of(
        this->reveals.cbegin(),
        this->reveals.cend(),
        [available_cubes] (Reveal reveal) {return reveal.IsRevealPossible(available_cubes);}
    );

    return are_all_reveals_possible;
}
