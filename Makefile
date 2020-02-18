img.out = img.gif

CONV = magick
ifeq (, $(shell which magick))
	CONV = convert
endif

DISPLAY = imdisplay
ifeq (, $(shell which imdisplay))
	DISPLAY = display
endif

all: test display 
	
test: 
	cargo test -- --nocapture --test-threads=1

display: gen
	$(DISPLAY) $(img.out)

gen: run
	$(CONV) img{0..9}.ppm $(img.out)
	

run:
	cargo run


clean:
	cargo clean
	rm -f *.ppm *.png log.txt $(img.out)