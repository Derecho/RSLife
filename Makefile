RUSTC=rustc
RUSTOPTIONS=-O -C prefer-dynamic -C link-args=-s -L .

.PHONY: all clean distclean

all: rslife

external/drawille:
	@mkdir -p external
	git clone -q https://github.com/P1start/drawille-rs.git external/drawille

libdrawille: external/drawille
	$(RUSTC) $(RUSTOPTIONS) external/drawille/src/drawille.rs
	@touch $@

librslife: libdrawille
	$(RUSTC) $(RUSTOPTIONS) src/rslife/mod.rs
	@touch $@

rslife: librslife 
	$(RUSTC) $(RUSTOPTIONS) src/main.rs -o $@

clean: 
	@rm -rf rslife target/main

distclean: clean
	@rm -rf libdrawille librslife lib*.rlib target/ external/
