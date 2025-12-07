R2EMU_DIR = $(shell pwd)
SCRIPTS = $(R2EMU_DIR)/scripts
TOOLS = $(R2EMU_DIR)/tools

# Kconfig
KCONFIG_FILE = $(R2EMU_DIR)/Kconfig
KCONFIG_DIR = $(TOOLS)/kconfig
KCONFIG_CONF = $(KCONFIG_DIR)/build/conf
KCONFIG_MCONF = $(KCONFIG_DIR)/build/mconf

all: build

$(KCONFIG_MCONF):
	$(MAKE) -C $(KCONFIG_DIR) NAME=mconf

$(KCONFIG_CONF):
	$(MAKE) -C $(KCONFIG_DIR)

# .config 规则
.config: $(KCONFIG_MCONF) $(KCONFIG_FILE)
	@echo ">>> Generating .config"
	@$(MAKE) menuconfig

build: .config
	@cargo build

build-release: .config
	@cargo build --release

run: build
	cargo run

clean:
	@echo CARGO CLEAN
	@cargo clean

distclean:
	@cargo clean
	rm -f .config
	$(MAKE) -C $(KCONFIG_DIR) clean

menuconfig: $(KCONFIG_MCONF)
	$(KCONFIG_MCONF) $(KCONFIG_FILE)

.PHONY: menuconfig build run clean distclean build-release
