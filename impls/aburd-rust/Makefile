step0_repl step1_read_print: clean
	cargo build --bin "${@}"
	cp "target/debug/${@}" "${@}"

clean:
	rm -f step*
