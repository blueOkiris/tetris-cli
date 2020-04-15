#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include <math.h>

#include <tetromino.h>
#include <canvas.h>
#include <game.h>
#include <key.h>

tetromino_t g_current_shape;
char        g_quit;
canvas_t    g_canvas;
float       g_fall_spd;
int         g_fall_key_delay;
int         g_score = 0;

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
    g_canvas = init_canvas((10 + 2) * SHAPE_WIDTH, 22 + 1);

    // Draw a border around the screen
    char border[g_canvas.width * g_canvas.height];
    setup_border((char *) border, g_canvas.width * g_canvas.height);

    g_quit = 0;
    char *clear_piece, *image_data;
    g_fall_spd = INITIAL_FALL;
    g_fall_key_delay = FALL_KEY_DELAY;
    g_score = 0;

    while(!g_quit) {
        clear_piece = m_get_tetromino_clear_image(g_current_shape);
        draw_image_to_canvas(&g_canvas, SHAPE_WIDTH * (g_current_shape.x - 2), g_current_shape.y - 2, SHAPE_WIDTH * 5, 5, clear_piece);

        update_piece();

        image_data = m_get_tetromino_image(g_current_shape);
        draw_image_to_canvas(&g_canvas, 0, 0, g_canvas.width, g_canvas.height, (char *) border);
        draw_image_to_canvas(&g_canvas, SHAPE_WIDTH * (g_current_shape.x - 2), g_current_shape.y - 2, SHAPE_WIDTH * 5, 5, image_data);
        draw_score();

        print_canvas(g_canvas);

        free(clear_piece);
        free(image_data);
        usleep(DELAY);
    }

    cleanup_canvas(g_canvas);
}

void update_piece() {
    /*if(g_current_shape.y > g_canvas.height - 2 - 2)
        g_current_shape = select_shape();*/
    if(keyboard_hit()) {
        int key = get_char();

        if(key == 127)                                              // Exit - delete key
            g_quit = 1;
        else if(key == 'a' && can_move(LEFT))
            g_current_shape.x--;
        else if(key == 'd' && can_move(RIGHT))
            g_current_shape.x++;
        else if(key == 'q' && can_rotate(LEFT))
            rotate_tetromino(&g_current_shape, 0);
        else if(key == 'e' && can_rotate(RIGHT))
            rotate_tetromino(&g_current_shape, 1);
        else if(key == 's') {
            if(can_move(DOWN))
                g_current_shape.y++;
            else {
                // Shape has landed                
                // Save current shape
                char *image_data = m_get_tetromino_image(g_current_shape);
                draw_image_to_canvas(&g_canvas, SHAPE_WIDTH * (g_current_shape.x - 2), ((int) g_current_shape.y) - 2, SHAPE_WIDTH * 5, 5, image_data);
                free(image_data);

                check_rows();

                // Select new shape
                g_current_shape = select_shape();
            }
        }
        
        //printf("%d\r\n", key);
    }

    if(can_move(DOWN))
        g_current_shape.y += g_fall_spd;
    else if(g_fall_key_delay > 0) {
        g_fall_key_delay--;
    } else {
        // Shape has landed
        // Save current shape
        char *image_data = m_get_tetromino_image(g_current_shape);
        draw_image_to_canvas(&g_canvas, SHAPE_WIDTH * (g_current_shape.x - 2), ((int) g_current_shape.y) - 2, SHAPE_WIDTH * 5, 5, image_data);
        free(image_data);

        check_rows();

        // Select new shape
        g_current_shape = select_shape();
        g_fall_key_delay = FALL_KEY_DELAY;
    }
}

// Check for rows and delete them if full
void check_rows() {
    for(int y = 2; y < g_canvas.height - 1; y++) {
        char row_full = 1;

        for(int x = 4; x < g_canvas.width - 4; x++) {
            if(g_canvas.draw_buffer[y * g_canvas.width + x] == ' ') {
                row_full = 0;
                break;
            }
        }

        if(row_full) {                                              // YAY POINTS!!
            // Update game speed and score
            g_score++;
            g_fall_spd += FALL_INC;

            // Clear row
            for(int x = 4; x < g_canvas.width - 4; x++)
                g_canvas.draw_buffer[y * g_canvas.width + x] = ' ';

            // bring all other rows down
            for(int y2 = y; y2 >= 2; y2--) {
                for(int x = 4; x < g_canvas.width - 4; x++)
                    g_canvas.draw_buffer[y2 * g_canvas.width + x] = g_canvas.draw_buffer[(y2 - 1) * g_canvas.width + x];
            }
        }
    }
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

char can_rotate(direction_t dir) {
    if(dir == UP || dir == DOWN)
        return 0;
    
    // Create a temporary shape and rotate it
    tetromino_t temp_shape;
    temp_shape.x = g_current_shape.x;
    temp_shape.y = g_current_shape.y;
    temp_shape.shape = g_current_shape.shape;
    
    for(int i = 0; i < 4; i++) {
        temp_shape.coords[i][0] = g_current_shape.coords[i][0];
        temp_shape.coords[i][1] = g_current_shape.coords[i][1];
    }

    rotate_tetromino(&temp_shape, (char) (dir - LEFT));

    // Generate its draw code and see if it overlaps anything
    char *draw_data = m_get_tetromino_image(temp_shape);
    char canvas_chunk[5 * (5 * SHAPE_WIDTH)];

    for(int y = 0; y < 5; y++) {
        for(int x = 0; x < 5; x++) {
            int g_canvas_x = (g_current_shape.x - 2) + x;
            int g_canvas_y = ((g_current_shape.y - 2) + y) + 1; // down
            int g_canvas_ind = g_canvas_y * g_canvas.width + (g_canvas_x * SHAPE_WIDTH);
            
            canvas_chunk[y * (SHAPE_WIDTH * 5) + (x * SHAPE_WIDTH)] =       g_canvas.draw_buffer[g_canvas_ind];
            canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 1)] = g_canvas.draw_buffer[g_canvas_ind + 1];
            canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 2)] = g_canvas.draw_buffer[g_canvas_ind + 2];
            canvas_chunk[y * (SHAPE_WIDTH * 5) + ((x * SHAPE_WIDTH) + 3)] = g_canvas.draw_buffer[g_canvas_ind + 3];
        }
    }

    char ret = !overlap(draw_data, (char *) canvas_chunk);
    free(draw_data);

    return ret;
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

tetromino_t select_shape() {
    tetromino_t new_shape;
    new_shape.x = 5;
    new_shape.y = 3;
    new_shape.shape = rand() % 7;

    for(int i = 0; i < 4; i++) {
        new_shape.coords[i][0] = g_shape_coords[new_shape.shape][i][0];
        new_shape.coords[i][1] = g_shape_coords[new_shape.shape][i][1];
    }
    
    return new_shape;
}

void draw_score() {
    g_canvas.draw_buffer[29] = 'S';
    g_canvas.draw_buffer[30] = 'C';
    g_canvas.draw_buffer[31] = 'O';
    g_canvas.draw_buffer[32] = 'R';
    g_canvas.draw_buffer[33] = 'E';
    g_canvas.draw_buffer[35] = '=';

    // Get the first 5 digits of the score
    char digits[5];
    int score = g_score;

    for(int i = 0; i < 5; i++) {
        digits[i] = (score % 10) + '0';
        score /= 10;
    }
    
    g_canvas.draw_buffer[37] = digits[4];
    g_canvas.draw_buffer[38] = digits[3];
    g_canvas.draw_buffer[39] = digits[2];
    g_canvas.draw_buffer[40] = digits[1];
    g_canvas.draw_buffer[41] = digits[0];
}

void setup_border(char *border, int size) {
    for(int i = 0; i < g_canvas.width * g_canvas.height; i++)
        border[i] = -1;
    border[6] = 'T';
    border[8] = 'E';
    border[10] = 'T';
    border[12] = 'R';
    border[14] = 'I';
    border[16] = 'S';
    for(int i = 0; i < g_canvas.width; i += 4) {
        border[(g_canvas.height - 1) * g_canvas.width + i] = '[';
        border[(g_canvas.height - 1) * g_canvas.width + i + 1] = '<';
        border[(g_canvas.height - 1) * g_canvas.width + i + 2] = '>';
        border[(g_canvas.height - 1) * g_canvas.width + i + 3] = ']';
    }
    for(int i = 0; i < g_canvas.height - 1; i++) {
        border[i * g_canvas.width] = '[';
        border[i * g_canvas.width + 1] = '<';
        border[i * g_canvas.width + 2] = '>';
        border[i * g_canvas.width + 3] = ']';

        border[i * g_canvas.width + (g_canvas.width - 4)] = '[';
        border[i * g_canvas.width + (g_canvas.width - 4) + 1] = '<';
        border[i * g_canvas.width + (g_canvas.width - 4) + 2] = '>';
        border[i * g_canvas.width + (g_canvas.width - 4) + 3] = ']';
    }
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