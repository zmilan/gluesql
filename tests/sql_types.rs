mod helper;

use helper::{Helper, SledHelper};

#[test]
fn sql_types() {
    let helper = SledHelper::new("data/sql_types");

    let create_sql = "
        CREATE TABLE Item (
            id INTEGER,
            content TEXT,
            verified BOOLEAN,
            ratio FLOAT
        );
    ";

    helper.run_and_print(create_sql);

    let delete_sql = "DELETE FROM Item";

    helper.run_and_print(delete_sql);

    let insert_sqls = [
        "INSERT INTO Item (id, content, verified, ratio) VALUES (1, \"Hello\", True, 0.1);",
        "INSERT INTO Item (id, content, verified, ratio) VALUES (1, \"World\", False, 0.9);",
    ];

    for insert_sql in insert_sqls.iter() {
        helper.run(insert_sql).unwrap();
    }

    let test_sqls = [
        (2, "SELECT * FROM Item;"),
        (1, "SELECT * FROM Item WHERE verified = True;"),
        (1, "SELECT * FROM Item WHERE ratio > 0.5;"),
        (1, "SELECT * FROM Item WHERE ratio = 0.1;"),
        (
            1,
            "UPDATE Item SET content=\"Foo\" WHERE content=\"World\";",
        ),
        (0, "SELECT * FROM Item WHERE content=\"World\";"),
        (1, "SELECT * FROM Item WHERE content=\"Foo\";"),
        (1, "UPDATE Item SET id = 11 WHERE content=\"Foo\";"),
        (2, "SELECT * FROM Item;"),
    ];

    for (num, sql) in test_sqls.iter() {
        helper.test_rows(sql, *num);
    }

    helper.run_and_print(delete_sql);
}
