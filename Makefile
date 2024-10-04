banner=printf "\033[35m"; banner $@; print "\033[0m";

test: build start
	make check || (make stop && echo "Tests failed" && exit 1)
	make stop

build:
	@$(banner)
	cargo build --bin client
	cargo build --bin server

start:
	@$(banner)
	cargo run --bin server &; echo $$! > /tmp/doordb.pid
	sleep 1

check:
	@$(banner)
	./target/debug/client counter create a | grep 0
	./target/debug/client counter read a | grep 0
	./target/debug/client counter increment a | grep 1
	./target/debug/client counter read a | grep 1
	./target/debug/client counter delete a | grep 1
	sudo ./target/debug/client counter create b
	./target/debug/client counter read b 2>&1 | grep EPERM
	./target/debug/client text write c hello | grep -v '*'
	./target/debug/client text read c | grep hello
	./target/debug/client text write c bye | grep hello
	./target/debug/client text read c | grep bye
	./target/debug/client text delete c | grep bye
	./target/debug/client text read c 2>&1 | grep "Key not found"

stop:
	@$(banner)
	if [ -e /tmp/doordb.pid ]; then kill `cat /tmp/doordb.pid`; fi
	rm -f /tmp/doordb
	rm -f /tmp/doordb.pid
