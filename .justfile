# Justfile (Convenience Command Runner)

# Convenience Variables
# rust vars
RUST_LOG := 'debug'
RUST_BACKTRACE := '1'
NO_WARN := '-Awarnings'
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

# Ready all local `.rs` files.
[confirm("This will:\n                                                                \
(1) Give user executable permissions to all `.rs` files in current directory level.\n \
    (`chmod u+x`)\n                                                                   \
(2) Run `cargo` clean, build, and doc on those files.\n\n                             \
Commands can be inspected in the currently invoked `justfile`.\n\n                    \
-- Confirm initialization?")]
init: _permit-all (cargo-script-all 'clean') _compile-debug _compile-release (cargo-script-all 'doc')

# Cargo _ on script file.
cargo-script command file *args:
    cargo +nightly {{command}} {{args}} --manifest-path {{file}} -Zscript

# Cargo _ on ALL `.rs` files at current directory level.
cargo-script-all command *args:
    fd . --extension rs --max-depth 1                                 \
        | xargs -I _                                                  \
        cargo +nightly {{command}} {{args}} --manifest-path _ -Zscript;

# New script, with executable user privileges.
[group('create')]
new name:
    cat .support/_template-script-basic_rs    \
        | sd '\{\{sd_me:(.*?)\}\}' '{{name}}' \
        > {{name}}.rs                         ;
    chmod u+x {{name}}.rs

# New script, with executable user privileges.
[group('create')]
new-clap name:
    cat .support/_template-script-clap_rs      \
        | sd '\{\{sd_me:(.*?)\}\}' '{{name}}' \
        > {{name}}.rs                         ;
    chmod u+x {{name}}.rs

# Linting, formatting, typo checking, etc.
[group('general')]
check file:
    @echo '-- clippy @ {{file}} --'
    just cargo-script clippy {{file}} --all-targets --all-features
    @echo '-- tests @ {{file}} --'
    RUSTFLAGS={{NO_WARN}} just cargo-script test {{file}} --all-targets --all-features --quiet
    @echo '-- typos @ {{file}} --'
    typos ./{{file}}

# Show general use docs.
[group('general')]
docs-gen:
    rustup doc
    rustup doc --std

# Show docs for a script.
[group('general')]
docs file:
    just cargo-script doc {{file}} --open --document-private-items --all-features

# Modify shebang: run without flags. (default)
[group('modify')]
simple-script file:
    sd '\#!/usr/bin/env -S cargo .*$'               \
        '#!/usr/bin/env -S cargo +nightly -Zscript' \
        {{file}}.rs                                 ;

# Modify shebang: use`--quiet` when called directly.
[group('modify')]
quiet-script file:
    sd '\#!/usr/bin/env -S cargo .*$'                       \
        '#!/usr/bin/env -S cargo +nightly --quiet -Zscript' \
        {{file}}                                            ;

# Modify shebang: use `--release` when called directly.
[group('modify')]
heavy-script file:
    sd '\#!/usr/bin/env -S cargo .*$'                                             \
        '#!/usr/bin/env -S cargo +nightly -Zscript run --release --manifest-path' \
        {{file}}                                                                  ;

# Modify shebang: use `--release` & `--quiet` when called directly.
[group('modify')]
stable-script file:
    sd '\#!/usr/bin/env -S cargo .*$'                                                     \
        '#!/usr/bin/env -S cargo +nightly --quiet -Zscript run --release --manifest-path' \
        {{file}}                                                                          ;

# Run performance analysis on a package.
[group('general')]
perf-script file *args:
    hyperfine './{{file}} {{args}}' --warmup=3 --shell=none;
    @echo 'Not run: {{GRN}}samply{{NC}} {{PRP}}record --iteration-count=3 ./{{file}} {{args}};{{NC}}'
    @echo 'samply would respond: "{{BRN}}Profiling failed: Could not obtain the root task.{{NC}}"'

# Run a file when it changes.
[group('watch')]
watch file:
    watchexec --filter {{file}} \
        'clear; ./{{file}}'     ;

# Run a file, without warnings, when it changes.
[group('watch')]
watch-quiet file:
    watchexec --filter {{file}}                  \
        'clear; RUSTFLAGS={{NO_WARN}} ./{{file}}';

# Lint & test a file when it changes.
[group('watch')]
watch-check file:
    watchexec --filter {{file}} \
        'clear; just check {{file}}'

# Lint & test then run a file when it changes.
[group('watch')]
watch-check-run file:
    watchexec --filter {{file}}          \
        'clear; just check {{file}};     \
        echo '-- run ./{{file}} --';     \
        RUSTFLAGS={{NO_WARN}} ./{{file}}';

# `chmod u+x` on ALL `.rs` files at current directory level.
_permit-all:
    fd . --extension rs --max-depth 1 \
        | xargs -I _                  \
        chmod u+x _                   ;

# `chmod u-x` on ALL `.rs` files at current directory level.
_depermit-all:
    fd . --extension rs --max-depth 1 \
        | xargs -I _                  \
        chmod a-x _                   ;

# Compile in debug mode if NO `--release` in shebang
_compile-debug:
    just _has-shebang-no-release \
        | xargs -I _             \
        just cargo-script build _;

# Compile in release mode if `--release` in shebang
_compile-release:
    just _has-shebang-release              \
        | xargs -I _                       \
        just cargo-script build _ --release;

# List files withOUT release in the sehbang.
_has-shebang-no-release:
    -@just _has-rs                                                  \
        | xargs -I _                                                \
        rg '^#!.*cargo' --files-with-matches _                      \
        | xargs -I _                                                \
        rg '^(#!.*\-\-release|[^#]|$)' -vm 1 --files-with-matches _ ;


# List files with `--release` in shebang.
_has-shebang-release:
    -@just _has-rs                                    \
        | xargs -I _                                  \
        rg '^#!.*cargo' --files-with-matches _        \
        | xargs -I _                                  \
        rg '^#!.*\-\-release' --files-with-matches _  ;

# List `.rs` files
_has-rs:
    -@fd . --extension rs --max-depth 1

# Info about Rust-Compiler, Rust-Analyzer, Cargo-Clippy, and Rust-Updater.
_rust-meta-info:
    rustc --version
    rust-analyzer --version
    cargo-clippy --version
    rustup --version

# ######################################################################## #.

# Count all `{{_}}` vs `{{pat_}}`, show diff.
[group('meta-tests')]
_bracket-diff pat_prefix='sd_me:' file_globs='.support/_template*':
    @echo "{{{{"{{pat_prefix}}".*}}:"
    @rg '\{\{''{{pat_prefix}}''.*\}\}' {{file_globs}} \
        | wc -l                                       ;
    @echo "{{{{".*"}}:"
    @rg '\{\{.*\}\}' {{file_globs}} \
        | wc -l                     ;
    @echo "Difference:"
    @-rg '\{\{.*\}\}' {{file_globs}}       \
        | rg {{pat_prefix}} --invert-match \
        | uniq -c                          ;

# Show contents of `{{pat_}}`.
[group('meta-tests')]
_bracket-show pat_prefix='sd_me:' file_globs='.support/_template*':
    @echo '{{{{'{{pat_prefix}}'_}} in files {{file_globs}}:'
    @rg '\{\{''{{pat_prefix}}''.*\}\}' {{file_globs}}  \
        | sd '.*\{\{''{{pat_prefix}}''(.*)\}\}.*' '$1' \
        | sort                                         \
        | uniq -c                                      ;

# Inspect counts, ensure partitioning.
[group('meta-tests')]
_check-release-counts:
    just _has-shebang-release | wc -l
    just _has-shebang-no-release | wc -l
    just _has-cargo-shebang | wc -l
    just _has-rs | wc -l

# List files with `--release` in shebang.
_has-cargo-shebang:
    -@just _has-rs                              \
        | xargs -I _                            \
        rg '^#!.*cargo' --files-with-matches _  ;

# ######################################################################## #.

# Freeze! For your safety.
_freeze file:
	mv -iv {{file}} FROZE_{{sha256(file)}}_FROZE-{{file}} | rg {{file}}

# Unfreeze a file. (removes 'FROZE...FROZE-' tag from filename).
_thaw file:
	echo {{file}} | sd '{{FROZE_SHA_REGEX}}' '' | xargs mv -iv {{file}}

# Search local files through ice.
_arctic-recon iceless_name:
	fd --max-depth 1 '{{FROZE_SHA_REGEX}}{{iceless_name}}' | rg {{iceless_name}}

# ######################################################################## #.

# Speak Funny to Me!
_uu:
	echo {{uuid()}}

# Say my name.
_sha file:
	echo {{sha256_file(file)}}

# Example function for syntax reference.
_example-file-exists-test file:
    echo {{ if path_exists(file) == "true" { "hello" } else { "goodbye" } }}

# ######################################################################## #.
