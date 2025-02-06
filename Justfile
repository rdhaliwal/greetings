# Same as just compile, just an alias
alias build := compile

# Format all rust files
format:
      rustfmt **/*.rs

# Compile the app to a production binary
compile:
      cargo build --release

# Run the app
run:
      cargo run

# Delete target dir
clean:
      rm -rf target

# Install the app to /usr/local/bin
install: compile
      sudo cp ./target/release/greetings /usr/local/bin/greetings
