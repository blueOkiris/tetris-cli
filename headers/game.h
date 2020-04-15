#pragma once
#include <tetromino.h>          // For typedef

// This file includes the actual game loop and state for running tetris

// Important game variables
#define INITIAL_FALL    0.01
#define FALL_DECREASE   0.001
#define DELAY           10000    //us
#define FALL_KEY_DELAY  50

typedef enum { UP, DOWN, LEFT, RIGHT } direction_t;

// Game functions
void test_draw_tetrominos();
void play();

// Support functions for game
tetromino_t select_shape();
void update_piece();

char can_move(direction_t dir);
char can_rotate(direction_t dir);
char overlap(char *data_a, char *canvas_chunk);