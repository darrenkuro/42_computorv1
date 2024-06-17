NAME	:=	computor
CARGO	:=	cargo

RM		:=	/bin/rm -f

all: $(NAME)

$(NAME):
	$(CARGO) build --release

clean:
	$(CARGO) clean

fclean: clean
	$(RM) target
	$(RM) cargo.lock

re: fclean all

.PHONY : all clean fclean re
