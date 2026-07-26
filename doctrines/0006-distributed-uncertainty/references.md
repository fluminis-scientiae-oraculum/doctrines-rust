# References

- [RFC 9110, HTTP Semantics: idempotent methods](https://www.rfc-editor.org/rfc/rfc9110.html#name-idempotent-methods)
  defines HTTP method idempotency and explicitly scopes the concept to intended
  effect.
- [Apache Kafka design: delivery semantics](https://kafka.apache.org/documentation/#semantics)
  documents at-most-once, at-least-once, and Kafka's transaction-scoped
  exactly-once mechanisms.
- [CloudEvents specification](https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md)
  provides standardized event identity and source context useful for
  correlation.
- [Stripe API: idempotent requests](https://docs.stripe.com/api/idempotent_requests)
  is an authoritative example of key retention, parameter comparison, and
  response replay semantics for one API.
- [Amazon Builders' Library: making retries safe with idempotent APIs](https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/)
  explains production API identity and retry design.
- [Gray and Cheriton, “Leases: An Efficient Fault-Tolerant Mechanism for
  Distributed File Cache Consistency”](https://dl.acm.org/doi/10.1145/74850.74870)
  is foundational literature on time-bounded distributed authority.
- [PostgreSQL documentation: transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)
  grounds database observations and anomalies for relevant examples.

The doctrine adds an operational contract for explicit unknown states,
reconciliation evidence, scoped claims, audit causality, and review gates.
