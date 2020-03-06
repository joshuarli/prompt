name = prompt
flags=$(shell pkg-config --libs libgit2)

$(name): src/main.c
	gcc $(flags) -O2 src/main.c -o $(name)

fmt: src/main.c
	clang-format -i src/main.c

test: $(name)
	./test.sh

clean:
	rm -f $(name)

.PHONY: test clean
