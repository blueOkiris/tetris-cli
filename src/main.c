#include <stdio.h>

#include <game.h>
#include <key.h>

int main(int argc, char **args) {
    //printf("Hello, world!\n");
    //test_draw_tetrominos();

    set_conio_terminal_mode();

    while(!keyboard_hit()) {
        printf("NO KEYBOARD!!\r\n");
    }

    printf("Key pressed: %d.\n", get_char());

    //play();

    return 0;
}