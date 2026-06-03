def _python_extension_test_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name + ".sh")
    content = """#!/usr/bin/env bash
set -euo pipefail

workspace="${{TEST_SRCDIR}}/${{TEST_WORKSPACE}}"
extension="${{workspace}}/{extension}"
script="${{workspace}}/{script}"

cp "${{extension}}" "${{TEST_TMPDIR}}/material_color_py.so"
PYTHONPATH="${{TEST_TMPDIR}}" python3 "${{script}}"
""".format(
        extension = ctx.file.extension.short_path,
        script = ctx.file.script.short_path,
    )

    ctx.actions.write(
        output = executable,
        content = content,
        is_executable = True,
    )

    return [DefaultInfo(
        executable = executable,
        runfiles = ctx.runfiles(
            files = [
                ctx.file.extension,
                ctx.file.script,
            ],
            transitive_files = depset(transitive = [
                target.files
                for target in ctx.attr.data
            ]),
        ),
    )]

python_extension_test = rule(
    implementation = _python_extension_test_impl,
    attrs = {
        "data": attr.label_list(allow_files = True),
        "extension": attr.label(allow_single_file = True, mandatory = True),
        "script": attr.label(allow_single_file = [".py"], mandatory = True),
    },
    test = True,
)
