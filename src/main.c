#include <stdio.h>

#include <game.h>
#include <key.h>

int main(int argc, char **args) {
    //printf("Hello, world!\n");
    //test_draw_tetrominos();

    /*set_conio_terminal_mode();

    while(!keyboard_hit()) {
        printf("NO KEYBOARD!!\r\n");
    }

    printf("Key pressed: %d.\n", get_char());*/

    printf("Tetris CLI by Dylan Turner.\n");
    printf("Press enter to begin...\n");

    set_conio_terminal_mode();
    int key = 0;
    while(key != 13) {
        while(!keyboard_hit());
        key = get_char();
    }
    //reset_terminal_mode();

    play();

    return 0;
}