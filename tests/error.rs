mod helper;

use gluesql::{
    EvaluateError, ExecuteError, FilterContextError, JoinError, RowError, SelectError, StoreError,
    TableError, ValueError,
};
use helper::{Helper, SledHelper};

#[test]
fn error() {
    let helper = SledHelper::new("data/error");

    let sql = "DROP TABLE TableA";
    helper.test_error(sql, ExecuteError::QueryNotSupported.into());

    helper.run_and_print("CREATE TABLE TableA (id INTEGER);");
    helper.run_and_print("INSERT INTO TableA (id) VALUES (1);");

    let test_cases = vec![
        (StoreError::SchemaNotFound.into(), "SELECT * FROM Nothing;"),
        (
            SelectError::TooManyTables.into(),
            "SELECT * FROM TableA, TableB",
        ),
        (
            TableError::TableFactorNotSupported.into(),
            "SELECT * FROM TableA JOIN (SELECT * FROM TableB) as TableC ON 1 = 1",
        ),
        (
            JoinError::UsingOnJoinNotSupported.into(),
            "SELECT * FROM TableA JOIN TableA USING (id);",
        ),
        (
            JoinError::JoinTypeNotSupported.into(),
            "SELECT * FROM TableA CROSS JOIN TableA as A;",
        ),
        (
            EvaluateError::NestedSelectRowNotFound.into(),
            "SELECT * FROM TableA WHERE id = (SELECT id FROM TableA WHERE id = 2);",
        ),
        (
            FilterContextError::ValueNotFound.into(),
            "SELECT * FROM TableA WHERE noname = 1;",
        ),
        (
            RowError::LackOfRequiredColumn("id".to_owned()).into(),
            "INSERT INTO TableA (id2) VALUES (1);",
        ),
    ];

    test_cases
        .into_iter()
        .for_each(|(error, sql)| helper.test_error(sql, error));

    helper.run_and_print("CREATE TABLE TableB (id BOOL);");
    helper.test_error(
        "INSERT INTO TableB (id) VALUES (0);",
        ValueError::SqlTypeNotSupported.into(),
    );
}
