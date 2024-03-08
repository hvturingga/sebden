// application/src/macros.rs

#[macro_export]
macro_rules! create_instance {
    ($factory_func:ident, $pool:expr) => {
        application::factories::$factory_func($pool)
    };
}