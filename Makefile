.PHONY:	build

NAME=$(shell grep -m 1 '^name =' Cargo.toml | cut -d '"' -f 2)
core?=$(shell sysctl -n hw.ncpu)
at?=.
## 
## ===================
##  RUST MAKE COMMAND 
## ===================
## 
help:
	@sed -ne '/@sed/!s/## //p' $(MAKEFILE_LIST)

build: ## help building bin file to specific folder.
## 	at:	folder where the bin file at.
## 		`make build at=main`
## 	core: core of the cpu to build parallel (faster)
## 		`make build core=10`
## 
# 	@starex kill 2>/dev/null 
	@echo "-----------------------------------------"
	@echo "BUILDING $(NAME)..."
	@rm -rf ./target
	@rm -rf $(at)/$(NAME)
	cargo build -j $(core) --release
	@cp ./target/*/$(NAME) $(at)/$(NAME)
	@rm -rf ./target
	@chmod 700 $(at)/$(NAME)
	@cp $(at)/$(NAME) ~/ 
	@echo "SUCCESSFULLY BUILD $(NAME) AT $(at)."
	@echo "-----------------------------------------"
	@echo "INFO $(NAME)."
	@file $(at)/$(NAME)
	@ls -l $(at)/$(NAME)
	@echo "-----------------------------------------"
	@echo "TESTING $(NAME)..."
	@$(at)/$(NAME) help
	@echo "SUCCESSFULLY TEST $(NAME)"
	@codesign --force --deep --sign - ~/starex
	@echo "-----------------------------------------"

clean: ## help cleaning bin file to specific folder.
## 	at:	folder where the bin file at.
## 		`make clean`,
## 		`make clean at=main`
## 
	@echo "-----------------------------------------"
	@echo "SUCCESSFULLY REMOVE $(NAME) FROM $(at)"
	@echo "-----------------------------------------"
	@rm -rf $(at)/$(NAME)

## 
## ===================
## Julian Khoo, 2026
## 