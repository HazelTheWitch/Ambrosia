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
