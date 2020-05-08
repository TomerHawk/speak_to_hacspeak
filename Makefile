TARGET = cw_polymul

CHAIN ?= 41
CDEFS += -DCHAIN_SIZE=2
PLATFORM = CW308_STM32F4
# main ersetzen
SRC += main_rust_test.c
SRC += simpleserial_cw.c


# hier die C file path angeben und h files
VPATH += ./pmpq/common
EXTRAINCDIRS += pmpq/common
VPATH += pmpq/polymul
EXTRAINCDIRS += pmpq/polymul

EXTRAINCDIRS += target/debug
VPATH += target/debug
LD_LIBARY_PATH += -L./target/debug/ -lspeak_to_hacspec
FIRMWAREPATH = ./pmpq/chipwhisperer/hardware/victims/firmware
include $(FIRMWAREPATH)/Makefile.inc
