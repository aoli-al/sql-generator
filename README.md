# SQL Generator

The idea of this project is to quickly generate mock data given the table schema for testing.

The user will provide a sql file with `CREATE TABLE` query, the program will then generate the `INSERT` queries with randomized values. 

The following key types are support:
- Int
- Float

To build and run:
```bash
cargo run --package sql_gen --bin sql_gen

Usage: sql_gen <IN_FILENAME> <ROW> <OUT_FILENAME>
```

## Demo
```
cargo run --package sql_gen --bin sql_gen create_table_example.sql 10 out_demo.sql
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/sql_gen create_table_example.sql 10 out_demo.sql`
success!
```

Input `create_table_example.sql`
```
CREATE TABLE Example (
    key1 int,
    key2 float,
    key3 int,
);
```

Output `out_demo.sql`
```
INSERT INTO Example (key1,key2,key3) VALUES (570080915,0.3326017107395912,2041731471);
INSERT INTO Example (key1,key2,key3) VALUES (-1621816396,0.2504709833380463,-893641627);
INSERT INTO Example (key1,key2,key3) VALUES (2026367375,0.11187439747243,-973716815);
INSERT INTO Example (key1,key2,key3) VALUES (-397045788,0.45414665478686056,-1058334232);
INSERT INTO Example (key1,key2,key3) VALUES (-1271435317,0.21013061553028045,805791500);
INSERT INTO Example (key1,key2,key3) VALUES (174026881,0.39741794428100174,992559332);
INSERT INTO Example (key1,key2,key3) VALUES (-618916610,0.09780515473024087,-418273415);
INSERT INTO Example (key1,key2,key3) VALUES (1426348933,0.34229022904164663,2016457394);
INSERT INTO Example (key1,key2,key3) VALUES (-1754188540,0.9170772713421181,-386679943);
INSERT INTO Example (key1,key2,key3) VALUES (58019886,0.03461671459436921,1363563167);

```
  
