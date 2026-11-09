Before any SDK call, the first standalone linker-review invocation failed with
ENOENT because its local source-audit.json copy had not yet been populated by
the later adaptation helper. The native source audit already existed and was
qualified at large-class-tiles176/source-audit.json. Copying those exact bytes
into the new SDK root supplies the missing dependency. No code, library,
protocol, test or scientific observation changes, and no successful native or
SDK build is repeated. The original failed invocation remains in the tool log.
