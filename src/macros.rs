#[macro_export]
macro_rules! clamp {
    ($x: expr, $min: expr, $max: expr) => {
        {
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
        }
    }
}

#[macro_export]
macro_rules! add_system {
    ($world: expr, $system: expr) => {
        $world.add_system(std::boxed::Box::new($system))
    };
}

#[macro_export]
macro_rules! impl_components {
    ($name: ident) => {
        impl_component!($name);
        impl_any_component!($name);
    };
}

#[macro_export]
macro_rules! impl_any_component {
    ($name: ident) => {
        impl crate::ecs::AnyComponent for $name { }
    };
}

#[macro_export]
macro_rules! impl_component {
    ($name: ident) => {
        impl crate::ecs::Component for $name { }
    };
}