OUTPUT = $(PROJECT_ROOT)/pkg/brew/djs-bin.rb
TOML = $(PROJECT_ROOT)/Cargo.toml

.PHONY: $(OUTPUT)

$(OUTPUT):
	./gen-djs-bin.rb $(TOML) $(OUTPUT)
