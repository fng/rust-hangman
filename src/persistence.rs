extern crate sqlite;

pub fn experiment_with_db() -> Result<(), sqlite::Error>{
    let connection = sqlite::open("test.db")?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS users(name TEXT, age INTEGER);"
    )?;
    println!("hello");

    let state = connection.prepare("insert into users(name, age) values (:name, :age);")?
        .bind_by_name::<&str>(":name", "asdf")?
        .bind_by_name::<i64>(":age", 48)?
        .next()?;
    dbg!(&state);

    let mut statement = connection.prepare("select * from users;")?;
    while let sqlite::State::Row = statement.next()? {
        let user = statement.read::<String>(0)?;
        let age = statement.read::<i64>(1)?;
        println!("user: {} age: {}", user, age)
    }

    return Ok(());
}