NAME = conf
obj := build
SRCS += confdata.c expr.c preprocess.c symbol.c util.c
SRCS += lexer.lex.c parser.tab.c
CC = gcc
INC_PATH = -I.
CFLAGS = -DYYDEBUG $(INC_PATH)
DISTRO = $(shell cat /etc/os-release | grep PRETTY_NAME | sed 's/PRETTY_NAME=//')

OUT = $(obj)/$(NAME)
OBJS = $(patsubst %.c,$(obj)/%.o,$(SRCS))
vpath %.c . $(obj) lxdialog

ifeq ($(DISTRO),"Gentoo Linux")
LIBS += -ltinfo
endif

ifeq ($(NAME),conf)
SRCS += conf.c
else ifeq ($(NAME),mconf)
SRCS += mconf.c $(shell find lxdialog/ -name "*.c")
LIBS += -lncurses
else
$(error bad target=$(NAME))
endif

all: conf 

conf: $(OUT)

mconf: $(OUT)

clean:
	@echo + CLEAN $(obj)
	@rm -rf $(obj)

$(OUT): $(OBJS)
	@echo + LD $@
	@$(CC) -o $@ $(OBJS) $(LIBS)

$(obj)/%.o: %.c
	@mkdir -p $(dir $@)
	@echo + CC $@
	@$(CC) $(CFLAGS) -c $< -o $@

# YACC (Bison) 规则（保持原样）
$(obj)/lexer.lex.o: $(obj)/parser.tab.h
$(obj)/parser.tab.c $(obj)/parser.tab.h: parser.y
	@echo + YACC $@
	@bison -v $< --defines=$(obj)/parser.tab.h -o $(obj)/parser.tab.c

# LEX (Flex) 规则（保持原样）
$(obj)/lexer.lex.c: lexer.l $(obj)/parser.tab.h
	@echo + LEX $@
	@flex -o $@ $<

.PHONY: conf mconf clean
