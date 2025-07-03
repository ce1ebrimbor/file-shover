run-dev:
	cargo run -- --root test-sites/simple-portfolio --port 7878

# Define a reusable function for generating files
define generate-file
	@mkdir -p $(dir $(1))
	dd if=/dev/urandom of=$(1) bs=1M count=$(2)
endef

# Usage examples
generate-1GB:
	$(call generate-file,test-sites/large-files/1GB.bin,1024)

generate-500MB:
	$(call generate-file,test-sites/large-files/500MB.bin,512)

generate-100MB:
	$(call generate-file,test-sites/large-files/100MB.bin,100)

clean:
	rm -vf test-sites/large-files/*

# Alternative approach using pattern rules (commented out)
# %.bin:
# 	@mkdir -p $(dir $@)
# 	dd if=/dev/urandom of=$@ bs=1M count=$(SIZE)
# 
# Usage would be: make test-sites/large-files/myfile.bin SIZE=1024

# Benchmark commands
bench-all:
	cargo bench

bench-files:
	cargo bench --bench file_operations

bench-http:
	cargo bench --bench http_parsing

bench-response:
	cargo bench --bench response_building

bench-open:
	@echo "Opening benchmark results in browser..."
	@if command -v xdg-open > /dev/null; then \
		xdg-open target/criterion/report/index.html; \
	elif command -v open > /dev/null; then \
		open target/criterion/report/index.html; \
	else \
		echo "Please open target/criterion/report/index.html in your browser"; \
	fi

# Clean benchmark results
bench-clean:
	rm -rf target/criterion