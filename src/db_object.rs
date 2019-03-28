use failure::Error;

trait DbObject<T> {
    fn getall(&self) -> Result<Vec<T>, Error>;
    fn get(&self, id: u64) -> Result<T, Error>;
    fn add(&self, object: T) -> Result<(), Error>; 
    fn update(&self, object: T) -> Result<(), Error>; 
}