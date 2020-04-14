# Name of program
OBJNAME=  tetris-cli

# Source folders
SRCFLDRS= src
INCFLDRS= headers

# Compiler options
# gcc   -> Use the GNU CCompiler
# -Wall -> Show all warnings
# -g    -> Allow debugging
# -O2   -> Enable tail-recursion
CC=       gcc
CFLAGS=   -Wall -g -O2

# Actual compiler options from folders
SRC=      $(foreach folder,$(SRCFLDRS),$(wildcard $(folder)/*.c))
HFILES=   $(foreach folder,$(INCFLDRS),$(wildcard $(folder)/*.h))
INC=      $(addprefix -I,$(INCFLDRS))

# Compile
.PHONY : all
all : $(OBJNAME)

$(OBJNAME) : $(HFILES) $(SRC)
	$(CC) $(CFLAGS) $(INC) -o $(OBJNAME) $(SRC)
