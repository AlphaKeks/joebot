# List available recipes
help:
	@just --list

# Creates the container
build:
	docker-compose build joebot

# Creates the container if necessary and starts it
up:
	docker-compose up -d

# Stops the container
down:
	docker-compose down

# Restarts the container
restart:
	@just down
	@just up

# Runs the bot
run:
	RUST_LOG=WARN,joebot=DEBUG cargo run \
		--quiet \
		-- \
			--debug \
			--config ./config.toml

# Run tests
test:
	cargo test \
		--quiet \
		-- --nocapture
