use sea_orm::entity::prelude::*;
use sea_orm::{Condition, DbBackend, QueryTrait};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "table")]
pub struct Model {
    #[sea_orm(primary_key)]
    field1: u32,
    field2: String,
    field3: String,
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

fn main() {
    let value = "dog";
    let id = 1000000;
    let cond_first_query = Entity::find()
        .filter(
            Condition::any()
                .add(Column::Field2.eq(value))
                .add(Column::Field3.eq(value)),
        )
        .filter(Column::Field1.eq(id))
        .into_query();

    let cond_second_query = Entity::find()
        .filter(Column::Field1.eq(id))
        .filter(
            Condition::any()
                .add(Column::Field2.eq(value))
                .add(Column::Field3.eq(value)),
        )
        .into_query();

    println!("{:?}", cond_first_query);
    println!("{:?}", cond_second_query);

    println!(
        "{}",
        cond_first_query.to_string(sea_orm::sea_query::MysqlQueryBuilder)
    );
    println!(
        "{}",
        cond_second_query.to_string(sea_orm::sea_query::MysqlQueryBuilder)
    );
}
