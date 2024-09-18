all: build start test stop

build:
	@banner $@
	cargo build --bin doordb
	cargo build --bin doordbsvc

test:
	@banner $@
	cargo run --bin doordb -- --key a --method create
	cargo run --bin doordb -- --key a --method get
	cargo run --bin doordb -- --key a --method increment
	cargo run --bin doordb -- --key a --method get
	cargo run --bin doordb -- --key a --method delete

start:
	@banner $@
	cargo run --bin doordbsvc &; echo $$! > /tmp/doordb.pid
	sleep 1

stop:
	@banner $@
	if [ -e /tmp/doordb.pid ]; then kill `cat /tmp/doordb.pid`; fi
	rm -f /tmp/doordb
	rm -f /tmp/doordb.pid
