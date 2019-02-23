## Markdown new line

When I use [enter] at the end of a line, it means new paragraph in md format,  
but when I want to use a new line within a paraghraph, the standard way to use  
double [enter], but it looks weird viewing in plain markdown - it looks like  
double enter, or an enter between two paragraph.

So I found the way, what is accepted in Emacs and in Github:  
- Using **double space** at the end of a line means new line.

!Deprecated  
Not used features, but maybe useful in the future.

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

```rust
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

@azonosító/ID  T161/ID    K3841/ID   5500

# Design plan

```rust
pub struct Transaction {
    pub id: i32,
    pub debit: i32,
    pub credit: i32,
    pub payment: i32,
    pub event: String,
    pub time_made: i32
    pub time_created: i32
}
```

Folyamat:
- Zárás
- Nyitás (Feltétele a verzió váltásnak)

Funkciók
-! Több könyvelés kezelése
-! Kontír tétel hozzáadása
-! Bruttó elszámolás - csak hozzá adni lehet, törölni és módosítani nem
-! Nyitás
-! Zárás
-! Számlakeret listázása
-! Számlakeret szerkesztése
  -! Új létrehozása
  -! Törlés - csak! ha az egyenlege nulla
-! főkönyv lekérdezése hónap alapján
-! számla egyenleg lekérdezés
- főkönyvi karton lekérdezés
- karton kezelésa

# For GTK GUI development!

Use this command, to avoid compile error:
```sudo apt-get install libgtk-3-dev```bash
