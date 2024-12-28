# Justfile (Convenience Command Runner)

# rust vars
RUST_LOG:= 'debug'
RUST_BACKTRACE:= '1'
RUSTFLAGS:='--cfg tokio_unstable'
TOML_VERSION:=`rg '^version = ".*"' Cargo.toml | sd '.*"(.*)".*' '$1'`
# just path vars
HOME_DIR := env_var('HOME')
LOCAL_ROOT := justfile_directory()
INVOCD_FROM := invocation_directory()
INVOC_IS_ROOT := if INVOCD_FROM == LOCAL_ROOT { "true" } else { "false" }
# custom vars
FROZE_SHA_REGEX := 'FROZE_[a-fA-F0-9]{64}_FROZE-'
# ANSI Color Codes for use with echo command
NC := '\033[0m'     # No Color
CYN := '\033[0;36m' # Cyan
BLU := '\033[0;34m' # Blue
GRN := '\033[0;32m' # Green
PRP := '\033[0;35m' # Purple
BRN := '\033[0;33m' # Brown

# Default, lists commands.
_default:
        @just --list --unsorted


# Linting, formatting, typo checking, etc.
check:
    typos
    committed

# Show docs.
docs:
    rustup doc
    rustup doc --std

# New script, with executable user privileges
new name:
    cat _template-script-basic.rs > {{name}}.rs
    chmod u+x {{name}}.rs

# New script, with executable user privileges
new-clap name:
    cat _template-script-clap.rs > {{name}}.rs
    chmod u+x {{name}}.rs

# Run performance analysis on a package.
perf-script file *args:
    hyperfine './{{file}} {{args}}' --warmup=3 --shell=none;
    samply record --iteration-count=3 ./{{file}} {{args}};

# Info about Rust-Compiler, Rust-Analyzer, Cargo-Clippy, and Rust-Updater.
rust-meta-info:
    rustc --version
    rust-analyzer --version
    cargo-clippy --version
    rustup --version

# Watch file and run when it changes. (Useful to get diagnostics.)
watch file:
    watchexec --filter {{file}}.rs ./{{file}}.rs
# ######################################################################## #

# Print reminder: how to set env vars that propagate to child shells.
_remind-setenv:
    @ echo '{{GRN}}set -a{{NC}}; {{GRN}}source {{BLU}}.env{{NC}}; {{GRN}}set +a{{NC}}'

# ######################################################################## #

# Freeze! For your safety.
_freeze file:
	mv -iv {{file}} FROZE_{{sha256(file)}}_FROZE-{{file}} | rg {{file}}

# Unfreeze a file. (removes 'FROZE...FROZE-' tag from filename)
_thaw file:
	echo {{file}} | sd '{{FROZE_SHA_REGEX}}' '' | xargs mv -iv {{file}}

# Search local files through ice.
_arctic-recon iceless_name:
	fd --max-depth 1 '{{FROZE_SHA_REGEX}}{{iceless_name}}' | rg {{iceless_name}}


# ######################################################################## #

# Speak Funny to Me!
_uu:
	echo {{uuid()}}

# Say my name.
_sha file:
	echo {{sha256_file(file)}}

# Example function for syntax reference
_example-file-exists-test file:
    echo {{ if path_exists(file) == "true" { "hello" } else { "goodbye" } }}

# ######################################################################## #
