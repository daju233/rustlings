// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    // You can optionally experiment here.
}

struct ColorTupleStruct(/* TODO: Something goes here */);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        struct ColorClassicStruct {
            // TODO: Something goes here
            red:i32,
            green:i32,
            blue:i32,
        
        }
        // TODO: Instantiate a classic c struct!
        // let green =
        let green = ColorClassicStruct{
            red:0,
            green:255,
            blue:0
        };
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        struct ColorTupleStruct (i32,i32,i32);
        let green=ColorTupleStruct(0,255,0);
        // TODO: Instantiate a tuple struct!
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        struct UnitLikeStructs;
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
