banner=printf "\033[35m"; banner $@; print "\033[0m";

test: start
	make check || (make stop && echo "Tests failed" && exit 1)
	make stop

start:
	@$(banner)
	cargo build --bin doordbd
	cargo run --bin doordbd &; echo $$! > /tmp/doordb.pid
	sleep 1

check:
	@$(banner)
	cargo run --bin doordb create a
	cargo run --bin doordb get a
	cargo run --bin doordb increment a
	cargo run --bin doordb get a
	cargo run --bin doordb delete a
	sudo ./target/debug/doordb create b
	cargo run --bin doordb get b 2>&1 | grep EPERM

stop:
	@$(banner)
	if [ -e /tmp/doordb.pid ]; then kill `cat /tmp/doordb.pid`; fi
	rm -f /tmp/doordb
	rm -f /tmp/doordb.pid
