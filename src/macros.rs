#[macro_export]
macro_rules! clamp {
    ($x: expr, $min: expr, $max: expr) => {{
        let min = $min;
        let max = $max;
        let x = $x;

        if x <= min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }};
}

#[macro_export]
macro_rules! add_system {
    ($world: expr, $system: expr, $priority: expr) => {
        $world.add_system(std::boxed::Box::new($system), $priority)
    };
}

#[macro_export]
macro_rules! query {
    () => {
        $crate::ecs::Query::new()
    };

    ($t: ty) => {
        $crate::query!().include::<$t>()
    };

    ($t: ty, $($ts: ty),+) => {
        $crate::query!($($ts),+).include::<$t>()
    };

    ($world: expr, $($ts: ty),*) => {
        $world.query_entities(&$crate::query!($($ts),*))
    };
}

#[macro_export]
macro_rules! query_one {
    ($world: expr, $($ts: ty),+) => {
        $world.query_one_entity(&$crate::query!($($ts),+))
    };
}
