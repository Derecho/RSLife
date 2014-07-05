RUSTC=rustc
RUSTOPTIONS=-O -C prefer-dynamic -C link-args=-s -L .

all: rslife

external/drawille:
	mkdir -p external
	git clone https://github.com/P1start/drawille-rs.git external/drawille
	$(RUSTC) $(RUSTOPTIONS) external/drawille/src/drawille.rs

librslife: external/drawille
	$(RUSTC) $(RUSTOPTIONS) src/rslife/mod.rs

rslife: librslife 
	$(RUSTC) $(RUSTOPTIONS) src/main.rs -o $@

clean: 
	rm -rf rslife target/main

distclean: clean
	rm -rf lib*.rlib target/ external/
