#[salsa::jar]
pub struct Jar(a::MyInput);

mod a {
    use crate::Jar;

    #[salsa::input(jar = Jar)]
    pub struct MyInput {
        field: u32,
    }
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {}

impl Db for Database {}

fn main() {
    let mut db = Database::default();
    let input = a::MyInput::new(&mut db, 22);

    input.field(&db);
    input.set_field(salsa::Durability::LOW, &mut db).to(23);
}
