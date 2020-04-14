#include <stdlib.h>
#include <stddef.h>

#include <tetromino.h>

// T_SHAPE, L_SHAPE, MIRROR_L, S_SHAPE, Z_SHAPE, SQUARE, LINE
const int g_shape_coords[7][4][2] = {
    { { -1,  0 }, { 0,  0 }, {  1, 0 }, {  0, 1 } },            // T_SHAPE
    { { -1, -1 }, { 0, -1 }, {  0, 0 }, {  0, 1 } },            // L_SHAPE
    { {  1, -1 }, { 0, -1 }, {  0, 0 }, {  0, 1 } },            // MIRROR_L
    { {  0, -1 }, { 0,  0 }, {  1, 0 }, {  1, 1 } },            // S_SHAPE
    { {  0, -1 }, { 0,  0 }, { -1, 0 }, { -1, 1 } },            // Z_SHAPE
    { {  0,  0 }, { 1,  0 }, {  0, 1 }, {  1, 1 } },            // SQUARE
    { {  0, -1 }, { 0,  0 }, {  0, 1 }, {  0, 2 } }             // LINE
};

// Rotate the shape
void rotate_tetromino(tetromino_t *tetromino, char right) {
    if(tetromino->shape == SQUARE)
        return;

    for(int i = 0; i < 4; i++) {
        int x = tetromino->coords[i][0];
        int y = tetromino->coords[i][1];

        if(right) {
            tetromino->coords[i][0] = -y;                       // x[i] = -y[i]
            tetromino->coords[i][1] = x;                        // y[i] = x[i]
        } else {
            tetromino->coords[i][0] = y;                        // x[i] = y[i]
            tetromino->coords[i][1] = -x;                       // y[i] = -x[i]
        }
    }
}

char *m_get_tetromino_image(tetromino_t tetromino) {
    /*
     * Shape could be -2 to 2 in all directions (bc rotations)
     * so needs to be 3x4 shape
     *      -2  -1  0   1    2
     *    _____________________
     * -2 |   |   |   |   |   |
     *    _____________________
     * -1 |   |   |   |   |   |
     *    ---------------------
     *  0 |   |   |   |   |   |
     *    ---------------------
     *  1 |   |   |   |   |   |
     *    ---------------------
     *  2 |   |   |   |   |   |
     *    ---------------------
     * 
     * But each x position is actually 4 x positions bc "[00]" shape
     * 
     * so (3 * 4) * 4
     */
    char *shape_data = (char *) malloc(sizeof(char) * ((5 * SHAPE_WIDTH) * 5)); // Create a place to store data
    
    // Clear it out first
    for(int i = 0; i < (5 * SHAPE_WIDTH) * 5; i++)
        shape_data[i] = -1;

    // Map the coords to the shape_data
    for(int i = 0; i < 4; i++) {
        // Adjust for -1
        int x = tetromino.coords[i][0] + 2;
        int y = tetromino.coords[i][1] + 2;

        shape_data[y * (5 * SHAPE_WIDTH) + (x * 4)] = '[';
        shape_data[y * (5 * SHAPE_WIDTH) + ((x * 4) + 1)] = '0';
        shape_data[y * (5 * SHAPE_WIDTH) + ((x * 4) + 2)] = '0';
        shape_data[y * (5 * SHAPE_WIDTH) + ((x * 4) + 3)] = ']';
    }

    return shape_data;
}

// SEE GET IMAGE
char *m_get_tetromino_clear_image(tetromino_t tetromino) {
    char *shape_data = (char *) malloc(sizeof(char) * ((5 * SHAPE_WIDTH) * 5));
    for(int i = 0; i < (5 * SHAPE_WIDTH) * 5; i++)
        shape_data[i] = -1;
    for(int i = 0; i < 4; i++) {
        int x = tetromino.coords[i][0] + 2;
        int y = tetromino.coords[i][1] + 2;

        shape_data[y * (5 * SHAPE_WIDTH) + (x * 4)] = ' ';
        shape_data[y * (5 * SHAPE_WIDTH) + ((x * 4) + 1)] = ' ';
        shape_data[y * (5 * SHAPE_WIDTH) + ((x * 4) + 2)] = ' ';
        shape_data[y * (5 * SHAPE_WIDTH) + ((x * 4) + 3)] = ' ';
    }

    return shape_data;
}
