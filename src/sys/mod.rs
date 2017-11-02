// mod particle;
// pub use self::system::lml_system_t;
// pub use self::system::{lml_system, lml_system_free};

mod system;
pub use self::system::lml_system_t;
pub use self::system::{lml_system, lml_system_add_particle, lml_system_free};
