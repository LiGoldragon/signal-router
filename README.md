# signal-persona-router

Signal contract for Persona router-owned observations and relations.

The first use is introspection: `introspect` can ask the router for a
typed summary or message trace without opening `router.redb`.

All current request variants are observation queries, so they use the `Match`
root through `RouterRequest::signal_verb()`.
