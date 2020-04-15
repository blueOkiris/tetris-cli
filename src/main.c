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

    while(1) {
        printf("\033[2J");
        printf("Tetris CLI by Dylan Turner cerca 2020.\n\n");
        printf("Controls:\n");
        printf(" - a/d -> move left/right respectively\n");
        printf(" - q/e -> rotate left/right\n");
        printf(" - s   -> drop piece quickly\n");
        printf(" - backspace -> quit game\n");
        printf("\nPress enter to begin...\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

        set_conio_terminal_mode();

        int key = 0;
        while(key != 13) {
            while(!keyboard_hit());
            key = get_char();

            if(key == 127)
                goto quit;
        }

        play();
        reset_terminal_mode();
    }

    quit:
        reset_terminal_mode();

    return 0;
}