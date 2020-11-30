pub mod actions {
    pub trait Action {
        fn action(&self);
    }
}

pub mod objects {
    pub struct Object {
        name: String,
        pub exists: bool
    }

    pub fn create(name: String) -> Object {
        let object = Object {
            name: name,
            exists: true,
        };

        return object;
    }

    pub fn remove(object: &Object) {
        object.name == "";
        object.exists == false;
    }
}
