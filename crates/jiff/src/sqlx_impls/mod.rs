/*!
Implementations of [SQLx] traits for Jiff's datetime types.

This replaces the `jiff-sqlx` wrapper crate. Since the trait impls live in
Jiff itself, there's no need for wrapper types: `jiff::Timestamp`,
`jiff::civil::DateTime`, `jiff::civil::Date` and `jiff::civil::Time` can be
used directly with SQLx's `bind`, `query_as`, `Row::get` and so on. For
PostgreSQL, `jiff::Span` additionally implements `Decode` (but not `Encode`).

`Zoned` is intentionally not supported. See the comment below for why.

[SQLx]: https://docs.rs/sqlx/0.8
*/

// We currently don't support `Zoned` integration. To briefly explain why, a
// `Zoned` is _both_ a timestamp and a time zone. And it isn't necessarily a
// dumb time zone like `-05:00`. It is intended to be a real time zone like
// `America/New_York` or `Australia/Tasmania`.
//
// However, PostgreSQL doesn't really have a primitive type that specifically
// supports "timestamp with time zone." Its `TIMESTAMP WITH TIME ZONE` type is
// actually just a timestamp. So storing a `Zoned` there would silently drop
// the time zone. Storing it as RFC 9557 text would be lossless, but would
// violate expectations that a datetime is stored in a datetime field.
//
// Ref: https://github.com/launchbadge/sqlx/issues/3487#issuecomment-2636542379

#[cfg(feature = "sqlx-postgres")]
mod postgres;
#[cfg(feature = "sqlx-sqlite")]
mod sqlite;
