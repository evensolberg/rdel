#!/usr/bin/env just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

# https://github.com/casey/just

# VARIABLES
application := "rdel"

# ALIASES
alias b := build
alias br := buildr
alias bra := buildra
alias fmt := format
alias r := release

# SHORTCUTS AND COMMANDS

# Builds and documents the project - Default; runs if nothing else is specified
@default: check

# Check if it builds at all
@check: format
    cargo lcheck  --color 'always'

# Only compiles the project
@build: format changelog
   cargo lbuild --color 'always'

# Compile a release version of the project without moving the binaries
@buildr: format changelog
    cargo lbuild --release --color 'always'
    cargo strip

# Compile a release version of the project for Apple ARM64 without moving the binaries
@buildra: format changelog
    cargo lbuild --release --color 'always' --target aarch64-apple-darwin
    cargo strip --target aarch64-apple-darwin

# Cleans and builds again
@rebuild: format changelog
    cargo clean
    cargo lbuild --color 'always'

# Updates the CHANGELOG.md file
@changelog:
   git-cliff --output {{invocation_directory()}}/CHANGELOG.md

# Cleans up the project directory
@clean:
    cargo clean
    -rm tree.txt > /dev/null 2>&1
    -rm graph.png > /dev/null 2>&1
    -rm debug.txt > /dev/null 2>&1
    -rm trace.txt > /dev/null 2>&1
    -rm bom.txt > /dev/null 2>&1

# Rebuilds the changelog
@cliff: changelog

# Documents the project, builds and installs the release version, and cleans up
@release: format changelog
    cargo lbuild --release  --color 'always'
    cargo strip
    cp {{invocation_directory()}}/target/release/{{application}} /usr/local/bin/
    cargo clean

# Documents the project, builds and installs the release version, and cleans up
@releasea: format changelog
    cargo lbuild --release  --color 'always' --target aarch64-apple-darwin
    cargo strip --target aarch64-apple-darwin
    cp {{invocation_directory()}}/target/aarch64-apple-darwin/release/{{application}} /usr/local/bin/
    cargo clean

# Build the documentation
@doc:
    cargo doc --no-deps

# Documents the project
@docs: format
    cargo doc --no-deps
    cargo depgraph | dot -Tpng > graph.png
    cargo tree > tree.txt
    cargo bom > bom.txt
    tokei
    cargo outdated

# Documents the project and all dependencies
@doc-all: format
    cargo doc
    cargo depgraph | dot -Tpng > graph.png
    cargo tree > tree.txt
    cargo bom > bom.txt
    tokei
    cargo outdated

# Formats the project source files
@format:
    cargo fmt -- --emit=files

# Tests the project
@test:
    cargo test


# Checks the project for inefficiencies and bloat
@inspect: format doc lint
    cargo deny check
    cargo geiger
    cargo bloat

# Checks for potential code improvements
@lint:
    cargo lclippy

# Initialize directory for various services such as cargo deny
@init:
    cp ~/CloudStation/Source/_Templates/deny.toml {{invocation_directory()}}/deny.toml
    cp ~/CloudStation/Source/_Templates/main_template.rs {{invocation_directory()}}/src/main.rs
    cp ~/CloudStation/Source/_Templates/cliff.toml {{invocation_directory()}}/cliff.toml
    cargo add clap --features cargo color
    cargo add log
    cargo add env_logger
    echo "# {{application}}\n\n" > README.md
    git mit-install
    git mit-config lint enable subject-line-not-capitalized
    git mit-config lint enable subject-line-ends-with-period
    git mit-config lint enable not-conventional-commit
    git mit-config lint enable not-emoji-log
    git mit-config mit set es "Even Solberg" even.solberg@gmail.com
    git mit es
    git remote add {{application}} https://github.com/evensolberg/{{application}}
    git commit -m doc:Initial
    git tag Initial
    git cliff --init

# Re-initialize the directory for various services -- stripped down version of init

@my_application:
    git mit-install
    git mit-config lint enable subject-line-not-capitalized
    git mit-config lint enable subject-line-ends-with-period
    git mit-config lint enable not-conventional-commit
    git mit-config lint enable not-emoji-log
    git mit-config mit set es "Even Solberg" even.solberg@gmail.com
    git mit es
    git cliff --init

# Read the documentation
@read:
    open file://{{invocation_directory()}}/target/doc/{{application}}/index.html

# Builds (if necessary) and runs the project
@run:
    cargo lrun  --color 'always'

# Build and run with a --help parameter
@runh:
    cargo lrun  --color 'always' -- --help

# Build and run with a --debug parameter
@rund:
    cargo lrun  --color 'always' -- --debug

# Build and run with a --debug parameter, tee to debug.txt
@rundt:
    cargo lrun  --color 'always' -- --debug | tee debug.txt

# Build and run with double --debug parameters
@rundd:
    cargo lrun  --color 'always' -- --debug --debug

# Build and run with double --debug parameters, tee to trace.txt
@runddt:
    cargo lrun  --color 'always' -- --debug --debug | tee trace.txt

# Check for new versions of crates and upgrade accordingly
@upgrade:
    cargo update
    cargo upgrade --workspace

# Copy this settings files to the templates directory
@just:
    -sed "s#{{application}}#my_application#" justfile > ~/CloudStation/Source/_Templates/justfile.template
    -cp {{invocation_directory()}}/deny.toml ~/CloudStation/Source/_Templates/deny.toml
    -cp {{invocation_directory()}}/cliff.toml ~/CloudStation/Source/_Templates/cliff.toml

# Check, but verbose
@checkv:
    cargo lcheck --color 'always' --verbose

# Install the relevant cargo add-ons used in this file
@install:
    -cargo install cargo-limit
    -cargo install cargo-geiger
    -cargo install cargo-depgraph
    -cargo install cargo-audit
    -cargo install cargo-bloat
    -cargo install cargo-edit
    -cargo install cargo-strip
    -cargo install --locked cargo-outdated
    -cargo install tokei
    -cargo install cargo-semver --vers 1.0.0-alpha.3
    -cargo install cargo-deny
    -brew tap git-chglog/git-chglog && brew install git-chglog
    -brew install PurpleBooth/repo/git-mit &&
    -cp ~/CloudStation/Source/_Templates/deny.toml {{invocation_directory()}}/deny.toml
    echo "Make sure to also install Graphviz."
