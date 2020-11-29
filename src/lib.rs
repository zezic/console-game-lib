pub mod actions {
    pub trait SimpleAction {
        fn action(&self);
    }

    pub trait ClassicAction {
        fn action(&self);
    }
}