# signal-router

Signal contract for Persona router-owned observations and relations.

The first use is introspection: `introspect` can ask the router for a
typed summary or message trace without opening `router.sema`.

`schema/lib.schema` is the authored source; `src/schema/lib.rs` is the
checked-in schema-rust generated contract surface. All current
request variants are observation queries carried as contract-local
`signal-frame` operation heads; durable read/write classification is
daemon-internal.
