all: clean server client

server:
	@echo "Building server..."
	rustc server.rs
	@echo ""

client:
	@echo "Building client..."
	rustc client.rs
	@echo ""

clean:
	@echo "Cleaning..."
	@rm -f server client
	@echo ""
