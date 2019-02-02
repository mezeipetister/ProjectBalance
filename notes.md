## Install DIESEL

I had a problem using Ubuntu 18.04.

I used this command:

> cargo install diesel_cli --force

but after an error message, I ended using this command:

> sudo apt install libpq-dev libmysqlclient-dev

Then everything worked fine.

## Timestamp handling with DIESEL

There was a problem handling timestamp. When I was using diesel::...get_results(), then an error occured, as in my models, I was using wrong type for timestamp - that was not implemented a few needed things.

Solution:

```
extern crate chrono;
use chrono::prelude::*;

pub time_created: NaiveDateTime


```

Related solution:

```
All of the types in diesel::sql_types are markers to represent various SQL datatypes for your schema. They should never be used in your own structs. What you need is a type which implements diesel::deserialize::FromSql<diesel::sql_types::Timestamp, diesel::pg::Pg> (docs: FromSql, Timestamp, Pg). There are two types which implement that trait.

The first is std::time::SystemTime which doesn't require additional dependencies, but doesn't have a ton of capabilities.

The second is chrono::NaiveDateTime. This is probably the type you want. In order to use it, you'll need to add chrono to your dependencies, and change the diesel line in Cargo.toml to include the chrono feature, so it'll look something like diesel = { version = "0.7.0", features = ["postgres", "chrono"] }

(Technically there's a third type, which is diesel::data_types::PgTimestamp but that's almost certainly not what you want, as that struct is just the literal representation of timestamp in the database so other types don't have to worry about raw bytes)

```

Made with (L) by Peter Mezei 2019.