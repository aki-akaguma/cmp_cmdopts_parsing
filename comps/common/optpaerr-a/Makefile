#!/usr/bin/make

empty:=
space:=$(empty) $(empty)
comma:=,

features = was_long single_error abbreviate subcommand long_only

features_comb = option_argument argument ambiguous subcommand option_argument+argument option_argument+ambiguous option_argument+subcommand argument+ambiguous argument+subcommand ambiguous+subcommand option_argument+argument+ambiguous option_argument+argument+subcommand option_argument+ambiguous+subcommand argument+ambiguous+subcommand

define template =
target/z.test-$(1).log: src/lib.rs
	time cargo test --no-default-features --features $(2) > z.test-$(1).tmp
	@cat z.test-$(1).tmp > target/z.test-$(1).log
	@grep -e "^test result:" z.test-$(1).tmp | grep -v -e " 0 passed" >> target/z.test-$(1).log
	@rm -f z.test-$(1).tmp
	@echo
endef

$(foreach log,$(features_comb),$(eval LOGS=$(LOGS) target/z.test-$(log).log))

all: $(LOGS)

clean:
	-@rm -f *.log
	-@rm -f *.tmp
	-@cargo clean

$(foreach log,$(features_comb),$(eval $(call template,$(log),$(subst +,$(comma),$(log)))))
