unexport CFLAGS
wiringpi:
	git submodule init
	git submodule update
	@echo >&2 $(CFLAGS)
	$(MAKE) -C wiringPi/wiringPi clean
	$(MAKE) -C wiringPi/wiringPi static CC=arm-linux-gnueabihf-gcc DEBUG=-O2
	rm -f $(OUT_DIR)/libwiringpi.a
	cp wiringPi/wiringPi/libwiringPi.a $(OUT_DIR)/libwiringpi.a

.PHONY: wiringpi