script;

fn main() {
    let num = 6;

    if num <= 1 {
        // do something
    } else if num <= 2 {
        // do something
    } else {
        // do something
    }

    // if - let
    let res = if num <= 1 {
        10
    } else {
        20
    };

    // match
    let x = 123;
    let s = match x {
        1 => "a",
        2 => "b",
        3 => "c",
        _ => "d",
    };

    // while loop
    let mut count = 0;
    while count < 10 {
        count += 1;

        // skip to next loop
        if count == 3 {
            continue;
        }

        // end loop
        if count == 7 {
            break;
        }
    }

}
