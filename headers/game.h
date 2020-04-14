#pragma once
#include <tetromino.h>          // For typedef

// This file includes the actual game loop and state for running tetris

// Important game variables
#define INITIAL_DELAY   700000  // 700000us = 700ms
#define DELAY_DECREASE  100000  // 100ms speed up

// Game functions
void test_draw_tetrominos();
void play();

// Support functions for game
tetromino_t select_shape();
void update_piece();

typedef enum { UP, DOWN, LEFT, RIGHT } direction_t;
char can_move(direction_t dir);
char overlap(char *data_a, char *canvas_chunk);