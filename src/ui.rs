pub fn welcome() {
    const WIDTH: i32 = 60;
    const HEIGHT: i32 = 15;

    const SIGN: &str = "#";
    const LETTER: &str = "*";

    let x_padding = 15;
    let y_padding = 3;

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || j == 0 || i == HEIGHT - 1 {
                print!("{}", SIGN)
            } else if (j == x_padding && i >= y_padding && i <= HEIGHT - y_padding)
                || (j == x_padding + 10 && i >= y_padding && i <= HEIGHT - y_padding - 5)
                || (i == y_padding && j >= x_padding && j <= x_padding + 10)
                || (i == y_padding + 4 && j >= x_padding && j <= x_padding + 10)
                || (j == x_padding + 8 && i >= y_padding + 5 && i <= HEIGHT - y_padding)
            {
                // R
                print!("{}", LETTER)
            } else if (j == WIDTH - x_padding - 10 && i >= y_padding && i <= HEIGHT - y_padding)
                || (i == y_padding && j >= WIDTH - x_padding - 10 && j <= WIDTH - x_padding)
                || (i == HEIGHT - y_padding
                    && j >= WIDTH - x_padding - 10
                    && j <= WIDTH - x_padding)
            {
                // C
                print!("{}", LETTER)
            } else {
                print!("{}", " ")
            }
            if j == WIDTH - 1 {
                print!("{}\n", SIGN)
            }
        }
    }
    let welcome_message = "> Welcome to RUST CLEANER <";
    let welcome_width = WIDTH - welcome_message.len() as i32;
    for i in 0..welcome_width {
        if i == welcome_width / 2 {
            print!("{}", welcome_message)
        } else {
            print!("=")
        }
    }
    println!("\ntype --help for help or instructions");
    println!("\n");
}
