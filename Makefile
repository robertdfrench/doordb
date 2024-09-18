all: start
	make test || (make stop && echo "Tests failed" && exit 1)
	make stop

start:
	@banner $@
	cargo build --bin doordbsvc
	cargo run --bin doordbsvc &; echo $$! > /tmp/doordb.pid
	sleep 1

test:
	@banner $@
	cargo run --bin doordb create a
	cargo run --bin doordb get a
	cargo run --bin doordb increment a
	cargo run --bin doordb get a
	cargo run --bin doordb delete a

stop:
	@banner $@
	if [ -e /tmp/doordb.pid ]; then kill `cat /tmp/doordb.pid`; fi
	rm -f /tmp/doordb
	rm -f /tmp/doordb.pid
