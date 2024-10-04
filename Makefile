banner=printf "\033[35m"; banner $@; print "\033[0m";

all: build start
	@make tests || (make stop && echo "Tests failed" && exit 1)
	@make stop

build:
	@$(banner)
	cargo build --bin client
	cargo build --bin server

start:
	@$(banner)
	./target/debug/server &; echo $$! > /tmp/doordb.pid
	sleep 1

stop:
	@$(banner)
	if [ -e /tmp/doordb.pid ]; then kill `cat /tmp/doordb.pid`; fi
	rm -f /tmp/doordb
	rm -f /tmp/doordb.pid

test: all

tests:
	@$(banner)

	# 01 - New counters begin at zero
	@./target/debug/client counter create a | grep 0 > /dev/null
	@./target/debug/client counter read a | grep 0 > /dev/null

	# 02 - Incrememnting a counter moves it up by 1
	@./target/debug/client counter increment a | grep 1 > /dev/null
	@./target/debug/client counter read a | grep 1 > /dev/null

	# 03 - Deleting a counter returns the current value
	@./target/debug/client counter delete a | grep 1 > /dev/null

	# 04 - Counters created by root are only readable by root
	@sudo ./target/debug/client counter create b > /dev/null
	@./target/debug/client counter read b 2>&1 | grep EPERM > /dev/null

	# 05 - Strings are empty before they are created
	@./target/debug/client text write c hello | grep -v '*' > /dev/null
	@./target/debug/client text read c | grep hello > /dev/null

	# 06 - Writing to a string returns its previous value
	@./target/debug/client text write c bye | grep hello > /dev/null
	@./target/debug/client text read c | grep bye > /dev/null

	# 07 - Deleting a string returns its previous value
	@./target/debug/client text delete c | grep bye > /dev/null

	# 08 - Deleted strings do not exist
	@./target/debug/client text read c 2>&1 | grep "Key not found" > /dev/null
