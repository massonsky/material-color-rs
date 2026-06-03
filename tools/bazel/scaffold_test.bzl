def _scaffold_test_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name + ".sh")
    ctx.actions.write(
        output = executable,
        content = """#!/usr/bin/env bash
set -euo pipefail

workspace="${TEST_SRCDIR}/${TEST_WORKSPACE}"

test -f "${workspace}/Cargo.toml"
test -f "${workspace}/MODULE.bazel"
test -f "${workspace}/crates/material_color/src/lib.rs"
""",
        is_executable = True,
    )

    return [DefaultInfo(
        executable = executable,
        runfiles = ctx.runfiles(files = ctx.files.data),
    )]

scaffold_test = rule(
    implementation = _scaffold_test_impl,
    attrs = {
        "data": attr.label_list(allow_files = True),
    },
    test = True,
)
