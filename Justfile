# Same as `just compile`, just an alias
alias build := compile

# Run the app
run:
      cargo run

# Format all rust files
format:
      rustfmt **/*.rs

# Delete target dir
clean:
      rm -rf target

# Compile the app to a production binary
compile:
      cargo build --release


# Install the app to /usr/local/bin
install: 
      sudo cp ./target/release/greetings /usr/local/bin/greetings

# Clean, Build and Install 
build-install: clean compile install
