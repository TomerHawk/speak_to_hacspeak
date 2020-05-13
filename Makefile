TARGET = cw_polymul
PLATFORM = CW308_STM32F4
# main ersetzen
SRC += main_rust_test.c
SRC += simpleserial_cw.c


# hier die C file path angeben und h files
VPATH += ./pmpq/common
EXTRAINCDIRS += pmpq/common

LDFLAGS += target/debug/libspeak_to_hacspec.a -Wl,--gc-section -lpthread -ldl
FIRMWAREPATH = ./pmpq/chipwhisperer/hardware/victims/firmware
include $(FIRMWAREPATH)/Makefile.inc
