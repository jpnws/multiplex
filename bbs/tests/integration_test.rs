#[test]
fn test_get_board_by_id() {
    use bbs::establish_connection;
    use bbs::models::Board;
    use bbs::schema::boards::dsl::boards;
    use diesel::prelude::*;
    let conn = &mut establish_connection();
    let result = boards.find(1).first::<Board>(conn);
    match result {
        Ok(board) => assert_eq!(board.name, "test"),
        Err(diesel::NotFound) => panic!("Board not found"),
        Err(err) => panic!("Database error: {}", err),
    }
}
