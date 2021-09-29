#pragma once

#define KEY_ESC     27
#define KEY_ENTER   13
#define KEY_LEFT    1

// Functions to set stdin to be character based so I can catch inputs
void reset_terminal_mode();
void set_conio_terminal_mode();
int keyboard_hit();
int get_char();