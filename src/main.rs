use ncurses::*;
use rand::Rng;
use random_word::Lang;
mod statistics;

const REGULAR_TEXT: i16 = 0;
const COMPLETED_TEXT: i16 = 1;
const WRONG_TEXT: i16 = 2;

const BACKSPACE_KEY: i32 = 127;
const ESCAPE_KEY: i32 = 27;

fn main() {
    initscr();
    create_colors();

    let mut stats_controller = statistics::Statistics::new();
    let original_text = get_random_words();

    while original_text.len() != stats_controller.current_text.len() {
        clear();

        // Render completed text
        let offset = stats_controller.current_text.len() + stats_controller.wrong_text.len();

        render_formatted_text(&stats_controller.current_text, COMPLETED_TEXT);
        render_formatted_text(&stats_controller.wrong_text, WRONG_TEXT);

        if offset < original_text.len() {
            render_formatted_text(&original_text[offset..].to_string(), REGULAR_TEXT);
        }

        addstr(&format!(
            "\n\nSpeed: {:.2} WPM \tAccuracy: {:.2}%",
            stats_controller.get_wpm(),
            stats_controller.get_accuracy()
        ));

        // Reposition the cursor
        mv(0, offset as i32);

        // Check the user inputs
        let key = getch();
        match key {
            BACKSPACE_KEY => stats_controller.delete_error(),
            ESCAPE_KEY => stats_controller.reset(),
            rest_keys => stats_controller.add_char(&original_text, rest_keys as u8 as char),
        }
    }

    endwin();
}

fn create_colors() {
    start_color();
    use_default_colors();
    init_pair(REGULAR_TEXT, COLOR_WHITE, -1);
    init_pair(COMPLETED_TEXT, COLOR_GREEN, -1);
    init_pair(WRONG_TEXT, COLOR_WHITE, COLOR_RED);
}

fn render_formatted_text(text: &String, format: i16) {
    attr_on(COLOR_PAIR(format));
    addstr(text);
    attr_off(COLOR_PAIR(format));
}

fn get_random_words() -> String {
    let mut vec = Vec::new();
    for _ in 0..20 {
        let length = rand::thread_rng().gen_range(3..7);
        vec.push(random_word::gen_len(length, Lang::En).unwrap())
    }

    vec.join(" ")
}
