# Justfile (Convenience Command Runner)

# Convenience Variables.
# rust vars
RUST_LOG:= 'debug'
RUST_BACKTRACE:= '1'
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
RED := '\033[0;31m' # Red
YLW := '\033[0;33m' # Yellow
BRN := '\033[0;33m' # Brown

# Default, lists commands.
_default:
        @just --list --unsorted

# Runs cargo command on a script file.
cargo-script file command *args:
    cargo +nightly {{command}} {{args}} --manifest-path {{file}}.rs -Zscript

# New script, with executable user privileges
[group('create')]
new name:
    cat _template-script-basic.rs | sd '\{\{sd_me:(.*?)\}\}' '{{name}}' > {{name}}.rs
    chmod u+x {{name}}.rs

# New script, with executable user privileges
[group('create')]
new-clap name:
    cat _template-script-clap.rs | sd '\{\{sd_me:(.*?)\}\}' '{{name}}' > {{name}}.rs
    chmod u+x {{name}}.rs

# Linting, formatting, typo checking, etc.
[group('general')]
check file:
    just cargo-script {{file}} check --all-targets --all-features
    just cargo-script {{file}} clippy --all-targets --all-features
    typos ./{{file}}.rs

# Show general use docs.
[group('general')]
docs-gen:
    rustup doc
    rustup doc --std

# Show docs for a script.
[group('general')]
docs file:
    just cargo-script {{file}} doc --open --document-private-items --all-features

# Run performance analysis on a package.
[group('general')]
perf-script file *args:
    hyperfine './{{file}}.rs {{args}}' --warmup=3 --shell=none;
    @echo 'Not run: {{GRN}}samply{{NC}} {{PRP}}record --iteration-count=3 ./{{file}}.rs {{args}};{{NC}}'
    @echo 'samply would respond: "{{BRN}}Profiling failed: Could not obtain the root task.{{NC}}"'

# Info about Rust-Compiler, Rust-Analyzer, Cargo-Clippy, and Rust-Updater.
_rust-meta-info:
    rustc --version
    rust-analyzer --version
    cargo-clippy --version
    rustup --version

# Run a file when it changes.
[group('watch')]
watch file:
    watchexec --filter {{file}}.rs 'clear; ./{{file}}.rs'

[group('watch')]
watch-check file:
    watchexec --filter {{file}}.rs 'clear; just cargo-script {{file}} check'

# Lint a file when it changes. (Can be quite noisy.)
[group('watch')]
watch-noisy-check file:
    watchexec --filter {{file}}.rs 'clear; just check {{file}}'

# Lint then run a file when it changes.
[group('watch')]
watch-noisy-run file:
    watchexec --filter {{file}}.rs 'clear; just check {{file}}; ./{{file}}.rs'

# ######################################################################## #

# Count all `{{_}}` vs `{{pat_}}`, show diff
[group('template_check')]
_bracket-diff pat_prefix='sd_me:' file_globs='_template*':
    @echo "{{{{"{{pat_prefix}}".*}}:"
    @rg '\{\{''{{pat_prefix}}''.*\}\}' {{file_globs}} | wc -l
    @echo "{{{{".*"}}:"
    @rg '\{\{.*\}\}' {{file_globs}} | wc -l
    @echo "Difference:"
    @-rg '\{\{.*\}\}' {{file_globs}} | rg {{pat_prefix}} --invert-match | uniq -c

# Show contents of `{{pat_}}`
[group('template_check')]
_bracket-show pat_prefix='sd_me:' file_globs='_template*':
    @echo '{{{{'{{pat_prefix}}'_}} in files {{file_globs}}:'
    @rg '\{\{''{{pat_prefix}}''.*\}\}' {{file_globs}} | sd '.*\{\{''{{pat_prefix}}''(.*)\}\}.*' '$1' | sort | uniq -c

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
