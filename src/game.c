#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>

#include <tetromino.h>
#include <canvas.h>
#include <game.h>

tetromino_t g_current_shape;
char        g_quit;
canvas_t    g_canvas;
int         g_delay;

// Start a new game
void play() {
    srand(time(0));
    g_current_shape = select_shape();

    /*
     * Tetris is a 10x20 grid (originally)
     * for this to look good, i'll make it 
     * 40x20 with blocks looking like [00]
     * 
     * thus 40x20
     */
    g_canvas = init_canvas(10 * SHAPE_WIDTH + 2, 22 + 2);

    // Draw a border around the screen
    char border[g_canvas.width * g_canvas.height];
    for(int i = 0; i < g_canvas.width * g_canvas.height; i++)
        border[i] = -1;
    for(int i = 1; i < g_canvas.width - 1; i++) {
        border[i] = '-';
        border[(g_canvas.height - 1) * g_canvas.width + i] = '-';
    }
    for(int i = 1; i < g_canvas.height - 1; i++) {
        border[i * g_canvas.width] = '|';
        border[i * g_canvas.width + (g_canvas.width - 1)] = '|';
    }

    g_quit = 0;
    char *clear_piece, *image_data;
    g_delay = INITIAL_DELAY;

    while(!g_quit) {
        clear_piece = m_get_tetromino_clear_image(g_current_shape);
        draw_image_to_canvas(&g_canvas, SHAPE_WIDTH * (g_current_shape.x - 2), g_current_shape.y - 2, SHAPE_WIDTH * 5, 5, clear_piece);

        update_piece();

        image_data = m_get_tetromino_image(g_current_shape);
        draw_image_to_canvas(&g_canvas, 0, 0, g_canvas.width, g_canvas.height, (char *) border);
        draw_image_to_canvas(&g_canvas, SHAPE_WIDTH * (g_current_shape.x - 2), g_current_shape.y - 2, SHAPE_WIDTH * 5, 5, image_data);

        print_canvas(g_canvas);

        free(clear_piece);
        free(image_data);
        usleep(g_delay);
    }

    cleanup_canvas(g_canvas);
}

char overlap(char *data_a, char *canvas_chunk) {
    for(int y = 0; y < 5; y++) {
        for(int x = 0; x < 5; x++) {
            if(data_a[(y * (5 * SHAPE_WIDTH)) + (x * SHAPE_WIDTH)] != -1 &&
                    canvas_chunk[(y * (5 * SHAPE_WIDTH)) + (x * SHAPE_WIDTH)] != ' ')
                return 1;
        }
    }

    return 0;
}

char can_move(direction_t dir) {
    char *image_data = m_get_tetromino_image(g_current_shape);
    char canvas_chunk[5 * (5 * SHAPE_WIDTH)];

    for(int y = 0; y < 5; y++) {
        for(int x = 0; x < 5; x++) {
            int g_canvas_x, g_canvas_y;
            int g_canvas_ind;

            switch(dir) {
                case DOWN:
                    g_canvas_x = (g_current_shape.x - 2) + x;
                    g_canvas_y = ((g_current_shape.y - 2) + y) + 1; // down
                    g_canvas_ind = g_canvas_y * g_canvas.width + (g_canvas_x * SHAPE_WIDTH);
                    
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + (x * SHAPE_WIDTH)] =       g_canvas.draw_buffer[g_canvas_ind];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 1)] = g_canvas.draw_buffer[g_canvas_ind + 1];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 2)] = g_canvas.draw_buffer[g_canvas_ind + 2];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 3)] = g_canvas.draw_buffer[g_canvas_ind + 3];
                    break;

                case UP:
                    g_canvas_x = (g_current_shape.x - 2) + x;
                    g_canvas_y = ((g_current_shape.y - 2) + y) - 1; // up
                    g_canvas_ind = g_canvas_y * g_canvas.width + (g_canvas_x * SHAPE_WIDTH);
                    
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + (x * SHAPE_WIDTH)] =       g_canvas.draw_buffer[g_canvas_ind];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 1)] = g_canvas.draw_buffer[g_canvas_ind + 1];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 2)] = g_canvas.draw_buffer[g_canvas_ind + 2];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 3)] = g_canvas.draw_buffer[g_canvas_ind + 3];
                    break;

                case LEFT:
                    g_canvas_x = (g_current_shape.x - 2) + x - 1;   // left
                    g_canvas_y = ((g_current_shape.y - 2) + y);
                    g_canvas_ind = g_canvas_y * g_canvas.width + (g_canvas_x * SHAPE_WIDTH);
                    
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + (x * SHAPE_WIDTH)] =       g_canvas.draw_buffer[g_canvas_ind];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 1)] = g_canvas.draw_buffer[g_canvas_ind + 1];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 2)] = g_canvas.draw_buffer[g_canvas_ind + 2];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 3)] = g_canvas.draw_buffer[g_canvas_ind + 3];
                    break;

                case RIGHT:
                    g_canvas_x = (g_current_shape.x - 2) + x + 1;   // right
                    g_canvas_y = ((g_current_shape.y - 2) + y);
                    g_canvas_ind = g_canvas_y * g_canvas.width + (g_canvas_x * SHAPE_WIDTH);
                    
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + (x * SHAPE_WIDTH)] =       g_canvas.draw_buffer[g_canvas_ind];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 1)] = g_canvas.draw_buffer[g_canvas_ind + 1];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 2)] = g_canvas.draw_buffer[g_canvas_ind + 2];
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 3)] = g_canvas.draw_buffer[g_canvas_ind + 3];
                    break;

                default:
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + (x * SHAPE_WIDTH)] =       -1;
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 1)] = -1;
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 2)] = -1;
                    canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 3)] = -1;
                    break;
            }
        }
    }

    char ret = !overlap(image_data, (char *) canvas_chunk);
    free(image_data);

    return ret;
}

void update_piece() {
    if(can_move(DOWN))
        g_current_shape.y++;
    else {
        // Shape has landed

        // Save current shape
        char *image_data = m_get_tetromino_image(g_current_shape);
        draw_image_to_canvas(&g_canvas, SHAPE_WIDTH * (g_current_shape.x - 2), g_current_shape.y - 2, SHAPE_WIDTH * 5, 5, image_data);
        free(image_data);

        // Select new shape
        g_current_shape = select_shape();
    }

    /*if(g_current_shape.y > g_canvas.height - 2 - 2)
        g_current_shape = select_shape();*/
}

tetromino_t select_shape() {
    tetromino_t new_shape;
    new_shape.x = 3;
    new_shape.y = 3;
    new_shape.shape = rand() % 7;

    for(int i = 0; i < 4; i++) {
        new_shape.coords[i][0] = g_shape_coords[new_shape.shape][i][0];
        new_shape.coords[i][1] = g_shape_coords[new_shape.shape][i][1];
    }
    
    return new_shape;
}

void test_draw_tetrominos() {
    /*
     * Tetris is a 10x20 grid (originally)
     * for this to look good, i'll make it 
     * 40x20 with blocks looking like [00]
     * 
     * thus 40x20
     */
    canvas_t canvas = init_canvas(10 * SHAPE_WIDTH, 20);

    for(int i = 0; i < 7; i++) {
        //char *test_data = "[00]    [00]    [00][00]";
        tetromino_t test_tetromino;
        test_tetromino.x = 3;
        test_tetromino.y = 5;
        test_tetromino.shape = i;
        for(int i = 0; i < 4; i++) {
            test_tetromino.coords[i][0] = g_shape_coords[test_tetromino.shape][i][0];
            test_tetromino.coords[i][1] = g_shape_coords[test_tetromino.shape][i][1];
        }

        char *tetromino_draw_data = m_get_tetromino_image(test_tetromino);
        clear_canvas(&canvas);
        draw_image_to_canvas(&canvas, SHAPE_WIDTH * (test_tetromino.x - 2), test_tetromino.y - 2, SHAPE_WIDTH * 5, 5, tetromino_draw_data);
        print_canvas(canvas);
        free(tetromino_draw_data);
        usleep(500000);

        // Test rotations
        rotate_tetromino(&test_tetromino, 1);
        tetromino_draw_data = m_get_tetromino_image(test_tetromino);
        clear_canvas(&canvas);
        draw_image_to_canvas(&canvas, SHAPE_WIDTH * (test_tetromino.x - 2), test_tetromino.y - 2, SHAPE_WIDTH * 5, 5, tetromino_draw_data);
        print_canvas(canvas);
        free(tetromino_draw_data);
        usleep(500000);
        rotate_tetromino(&test_tetromino, 1);
        tetromino_draw_data = m_get_tetromino_image(test_tetromino);
        clear_canvas(&canvas);
        draw_image_to_canvas(&canvas, SHAPE_WIDTH * (test_tetromino.x - 2), test_tetromino.y - 2, SHAPE_WIDTH * 5, 5, tetromino_draw_data);
        print_canvas(canvas);
        free(tetromino_draw_data);
        usleep(500000);
        rotate_tetromino(&test_tetromino, 1);
        tetromino_draw_data = m_get_tetromino_image(test_tetromino);
        clear_canvas(&canvas);
        draw_image_to_canvas(&canvas, SHAPE_WIDTH * (test_tetromino.x - 2), test_tetromino.y - 2, SHAPE_WIDTH * 5, 5, tetromino_draw_data);
        print_canvas(canvas);
        free(tetromino_draw_data);
        usleep(500000);
        rotate_tetromino(&test_tetromino, 1);
        tetromino_draw_data = m_get_tetromino_image(test_tetromino);
        clear_canvas(&canvas);
        draw_image_to_canvas(&canvas, SHAPE_WIDTH * (test_tetromino.x - 2), test_tetromino.y - 2, SHAPE_WIDTH * 5, 5, tetromino_draw_data);
        print_canvas(canvas);
        free(tetromino_draw_data);
        usleep(500000);

        rotate_tetromino(&test_tetromino, 0);
        tetromino_draw_data = m_get_tetromino_image(test_tetromino);
        clear_canvas(&canvas);
        draw_image_to_canvas(&canvas, SHAPE_WIDTH * (test_tetromino.x - 2), test_tetromino.y - 2, SHAPE_WIDTH * 5, 5, tetromino_draw_data);
        print_canvas(canvas);
        free(tetromino_draw_data);
        usleep(500000);
        rotate_tetromino(&test_tetromino, 0);
        tetromino_draw_data = m_get_tetromino_image(test_tetromino);
        clear_canvas(&canvas);
        draw_image_to_canvas(&canvas, SHAPE_WIDTH * (test_tetromino.x - 2), test_tetromino.y - 2, SHAPE_WIDTH * 5, 5, tetromino_draw_data);
        print_canvas(canvas);
        free(tetromino_draw_data);
        usleep(500000);
        rotate_tetromino(&test_tetromino, 0);
        tetromino_draw_data = m_get_tetromino_image(test_tetromino);
        clear_canvas(&canvas);
        draw_image_to_canvas(&canvas, SHAPE_WIDTH * (test_tetromino.x - 2), test_tetromino.y - 2, SHAPE_WIDTH * 5, 5, tetromino_draw_data);
        print_canvas(canvas);
        free(tetromino_draw_data);
        usleep(500000);
        rotate_tetromino(&test_tetromino, 0);
        tetromino_draw_data = m_get_tetromino_image(test_tetromino);
        clear_canvas(&canvas);
        draw_image_to_canvas(&canvas, SHAPE_WIDTH * (test_tetromino.x - 2), test_tetromino.y - 2, SHAPE_WIDTH * 5, 5, tetromino_draw_data);
        print_canvas(canvas);
        free(tetromino_draw_data);
        usleep(500000);
    }

    cleanup_canvas(canvas);
}