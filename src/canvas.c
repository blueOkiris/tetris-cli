#include <stdlib.h>
#include <stdio.h>

#include <canvas.h>

canvas_t init_canvas(int width, int height) {
    canvas_t new_canvas;

    new_canvas.width = width;
    new_canvas.height = height;

    // Fill the canvas buffer with ' '
    new_canvas.draw_buffer = (char *) malloc(sizeof(char) * ((width + 20) * (height + 5)));
    clear_canvas(&new_canvas);

    return new_canvas;
}

void cleanup_canvas(canvas_t canvas) {
    free(canvas.draw_buffer);
}

// Print out the canvas to stdin
void print_canvas(canvas_t canvas) {
    printf("\033[2J");                      // Move the cursor back to (0, 0) and clear the screen

    // Print out all the lines
    for(int y = 0; y < canvas.height; y++) {
        for(int x = 0; x < canvas.width; x++)
            printf("%c", canvas.draw_buffer[y * canvas.width + x]);
        
        printf("\r\n");
    }
}

// Place image data into the canvas
void draw_image_to_canvas(canvas_t *canvas, int x, int y, int width, int height, char *draw_data) {
    for(int cy = y; cy < y + height; cy++) {
        for(int cx = x; cx < x + width; cx++) {
            int canvas_ind = cy * canvas->width + cx;
            int draw_ind = (cy - y) * width + (cx - x);

            if(draw_data[draw_ind] != -1)   // -1 is the "skip"/"blank" character
                canvas->draw_buffer[canvas_ind] = draw_data[draw_ind];
        }
    }
}

void clear_canvas(canvas_t *canvas) {
    for(int i = 0; i < canvas->width * canvas->height; i++)
        canvas->draw_buffer[i] = ' ';
}