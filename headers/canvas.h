#pragma once

// This is a mini-library that sets up a console to work as a sort of ASCII graphics canvas
typedef struct {
    int width, height;
    char *draw_buffer;

} canvas_t;

canvas_t init_canvas(int width, int height);
void cleanup_canvas(canvas_t canvas);

void print_canvas(canvas_t canvas);
void draw_image_to_canvas(canvas_t *canvas, int x, int y, int width, int height, char *draw_data);
void clear_canvas(canvas_t *canvas);
