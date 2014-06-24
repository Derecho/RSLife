RUSTC=rustc
RUSTOPTIONS=-O -C prefer-dynamic -C link-args=-s
APP=gol

all: $(APP)

$(APP):
	$(RUSTC) $(RUSTOPTIONS) $(APP).rs

clean:
	rm $(APP)
