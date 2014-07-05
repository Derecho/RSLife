RUSTC=rustc
RUSTOPTIONS=-O -C prefer-dynamic -C link-args=-s

all: rslife

librslife:
	$(RUSTC) $(RUSTOPTIONS) src/rslife/mod.rs

rslife: librslife
	$(RUSTC) $(RUSTOPTIONS) -L . src/main.rs -o $@

clean:
	rm -rf rslife librslife*.rlib target/
