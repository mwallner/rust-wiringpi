unexport CFLAGS
wiringpi:
	@echo >&2 $(CFLAGS)
	$(MAKE) -C wiringPi/wiringPi clean
	$(MAKE) -C wiringPi/wiringPi static CC=arm-linux-gnueabihf-gcc DEBUG=-O2
	rm -f $(OUT_DIR)/libwiringpi.a
	cp wiringPi/wiringPi/libwiringPi.a $(OUT_DIR)/libwiringpi.a
wiringop:
	@echo >&2 $(CFLAGS)
	$(MAKE) -C WiringOP/wiringPi clean
	$(MAKE) -C WiringOP/wiringPi static CC=arm-linux-gnueabihf-gcc DEBUG=-O2
	rm -f $(OUT_DIR)/libwiringpi.a
	cp WiringOP/wiringPi/libwiringPi.a $(OUT_DIR)/libwiringpi.a

.PHONY: wiringpi wiringop
