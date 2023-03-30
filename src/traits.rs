pub trait Named {
 fn name(&self) -> &String;   
}

pub struct User {
    pub id:i32,
    pub name:String,
}

impl Named for User {
    fn name(&self)->String{
        // return &self.name;
        &self.name
    }
}