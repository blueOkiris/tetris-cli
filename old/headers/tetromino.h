#pragma once

#include <canvas.h> // For types

#define SHAPE_WIDTH 4

// Define a structure for the tetromino given position, shape, and rotation
typedef enum { T_SHAPE, L_SHAPE, MIRROR_L, S_SHAPE, Z_SHAPE, SQUARE, LINE } shape_t;

typedef struct {
    float x, y;     // Position of the shape

    shape_t shape;  // Type of tetromino
    int coords[4][2];
} tetromino_t;

extern const int g_shape_coords[7][4][2];

char *m_get_tetromino_image(tetromino_t tetromino);
char *m_get_tetromino_clear_image(tetromino_t tetromino);
void rotate_tetromino(tetromino_t *tetromino, char right);
