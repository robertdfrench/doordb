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
	cargo run --bin doordb counter create a
	cargo run --bin doordb counter read a
	cargo run --bin doordb counter increment a
	cargo run --bin doordb counter read a
	cargo run --bin doordb counter delete a
	sudo ./target/debug/doordb counter create b
	cargo run --bin doordb counter read b 2>&1 | grep EPERM

stop:
	@$(banner)
	if [ -e /tmp/doordb.pid ]; then kill `cat /tmp/doordb.pid`; fi
	rm -f /tmp/doordb
	rm -f /tmp/doordb.pid
