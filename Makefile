banner=printf "\033[35m"; banner $@; print "\033[0m";

test: build start
	make check || (make stop && echo "Tests failed" && exit 1)
	make stop

build:
	@$(banner)
	cargo build --bin doordb
	cargo build --bin doordbd

start:
	@$(banner)
	cargo run --bin doordbd &; echo $$! > /tmp/doordb.pid
	sleep 1

check:
	@$(banner)
	./target/debug/doordb counter create a | grep 0
	./target/debug/doordb counter read a | grep 0
	./target/debug/doordb counter increment a | grep 1
	./target/debug/doordb counter read a | grep 1
	./target/debug/doordb counter delete a | grep 1
	sudo ./target/debug/doordb counter create b
	./target/debug/doordb counter read b 2>&1 | grep EPERM
	./target/debug/doordb text write c hello | grep -v '*'
	./target/debug/doordb text read c | grep hello
	./target/debug/doordb text write c bye | grep hello
	./target/debug/doordb text read c | grep bye
	./target/debug/doordb text delete c | grep bye
	./target/debug/doordb text read c 2>&1 | grep "Key not found"

stop:
	@$(banner)
	if [ -e /tmp/doordb.pid ]; then kill `cat /tmp/doordb.pid`; fi
	rm -f /tmp/doordb
	rm -f /tmp/doordb.pid
