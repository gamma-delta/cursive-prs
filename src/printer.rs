//! Makes drawing on ncurses windows easier.

use ncurses;

use color;
use vec::{Vec2,ToVec2};

/// Convenient interface to draw on a subset of the screen.
pub struct Printer {
    /// Offset into the window this printer should start drawing at.
    pub offset: Vec2,
    /// Size of the area we are allowed to draw on.
    pub size: Vec2,
}

impl Printer {
    /// Creates a new printer on the given window.
    pub fn new<T: ToVec2>(size: T) -> Self {
        Printer {
            offset: Vec2::zero(),
            size: size.to_vec2(),
        }
    }

    /// Prints some text at the given position relative to the window.
    pub fn print<S: ToVec2>(&self, pos: S, text: &str) {
        let p = pos.to_vec2() + self.offset;
        ncurses::mvprintw(p.y as i32, p.x as i32, text);
    }

    /// Prints a vertical line using the given character.
    pub fn print_vline<T: ToVec2>(&self, start: T, len: u32, c: u64) {
        let p = start.to_vec2() + self.offset;
        ncurses::mvvline(p.y as i32, p.x as i32, c, len as i32);
    }

    /// Prints a horizontal line using the given character.
    pub fn print_hline<T: ToVec2>(&self, start: T, len: u32, c: u64) {
        let p = start.to_vec2() + self.offset;
        ncurses::mvhline(p.y as i32, p.x as i32, c, len as i32);
    }

    /// Returns a wrapper around this printer,
    /// that will apply the given style on prints.
    pub fn with_style<'a, F>(&'a self, style: color::ThemeColor, f: F)
        where F: Fn(&Printer)
    {
        ncurses::attron(ncurses::COLOR_PAIR(style));
        f(self);
        ncurses::attroff(ncurses::COLOR_PAIR(style));
        ncurses::attron(ncurses::COLOR_PAIR(color::PRIMARY));
    }

    /// Prints a rectangular box.
    ///
    /// # Examples
    ///
    /// ```
    /// printer.print_box((0,0), (6,4), '+', '-', '|');
    /// ```
    pub fn print_box<T: ToVec2>(&self, start: T, size: T) {
        let start_v = start.to_vec2();
        let size_v = size.to_vec2() - (1,1);

        self.print(start_v, "┌");
        self.print(start_v + size_v.keep_x(), "┐");
        self.print(start_v + size_v.keep_y(), "└");
        self.print(start_v + size_v, "┘");

        self.print_hline(start_v + (1,0), size_v.x - 1, ncurses::ACS_HLINE());
        self.print_vline(start_v + (0,1), size_v.y - 1, ncurses::ACS_VLINE());
        self.print_hline(start_v + (1,0) + size_v.keep_y(), size_v.x - 1, ncurses::ACS_HLINE());
        self.print_vline(start_v + (0,1) + size_v.keep_x(), size_v.y - 1, ncurses::ACS_VLINE());
    }

    /// Returns a printer on a subset of this one's area.
    pub fn sub_printer<S: ToVec2>(&self, offset: S, size: S) -> Printer {
        let offset_v = offset.to_vec2();
        Printer {
            offset: self.offset + offset_v,
            // We can't be larger than what remains
            size: Vec2::min(self.size - offset_v, size.to_vec2()),
        }
    }
}