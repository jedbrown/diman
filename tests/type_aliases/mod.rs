macro_rules! gen_tests_for_float {
    ($float_name: ident, $mod_name: ident) => {
        mod $mod_name {
            use crate::example_system::$float_name::Area;
            use crate::example_system::$float_name::Length;
            use crate::example_system::$float_name::Time;
            use crate::example_system::$float_name::Velocity;
            use crate::example_system::$float_name::Volume;
            use diman::Product;
            use diman::Quotient;

            fn product_1(length: Length, time: Time) -> Product<(Length, Time)> {
                length * time
            }

            fn quotient_1(length: Length, time: Time) -> Quotient<Length, Time> {
                length / time
            }

            fn quotient_2(
                length: Length,
                time: Time,
            ) -> Quotient<(Length, Time), (Length, Velocity)> {
                let vel: Velocity = length / time;
                length * time / (length * vel)
            }

            fn quotient_3(length: Length, time: Time) -> Quotient<(Length, Time), (Area, Volume)> {
                let vol: Volume = length.cubed();
                let area: Area = length.squared();
                length * time / (area * vol)
            }

            #[test]
            fn type_aliases() {
                // All of these really just need compile, so no need to check for equality. (In principle
                // we don't even need this test)
                product_1(Length::meters(2.0), Time::seconds(2.0));
                quotient_1(Length::meters(2.0), Time::seconds(2.0));
                quotient_2(Length::meters(2.0), Time::seconds(2.0));
                quotient_3(Length::meters(2.0), Time::seconds(2.0));
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(f32, f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(f64, f64);
