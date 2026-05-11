mod row;
mod version;

pub use row::*;
pub use version::*;

pub trait Table {
    type Id;
    type Data;

    fn exists(&self, id: &Self::Id) -> bool;
    fn get(&self, id: &Self::Id) -> Option<&Row<Self::Id, Self::Data>>;
    fn insert(&mut self, data: Self::Data) -> &Row<Self::Id, Self::Data>;
    fn update<P>(&mut self, id: &Self::Id, project: P)
    where
        P: FnOnce(&mut Self::Data);
    fn delete(&mut self, id: &Self::Id) -> Option<Row<Self::Id, Self::Data>>;
}

// pub trait Storage {
//     type Error;

//     fn read(&self, key: )
// }
