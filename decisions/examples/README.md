# Worked examples

Two illustrations of the decision-record test in
[RUST-DOC-0011](../../doctrines/0011-executable-narrative/): one decision whose residue justifies
a narrow record, and one whose does not.

Both are examples. Neither is a record, neither appears in
[`manifest/decision-records.yaml`](../../manifest/decision-records.yaml), and neither states an
obligation of this repository or of any real organization. Their identifiers deliberately do not
match the `ADR-NNNN` pattern the registry schema requires, so an attempt to register one fails
validation rather than succeeding quietly.

| Example                                                  | Outcome                                              |
| -------------------------------------------------------- | ---------------------------------------------------- |
| [Data residency](justified-data-residency.md)            | a record is written, narrowly, for the residue       |
| [Authentication order](rejected-authentication-order.md) | no record is written; the obligation becomes a bound |

The contrast is the point. The two decisions are comparable in perceived importance, and only one
of them contains a fact that no artifact can carry.
