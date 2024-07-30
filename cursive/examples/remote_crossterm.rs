use std::{
    fs::{self, File},
    io::{self, IsTerminal, Write},
    path::PathBuf,
};

use crossterm::tty::IsTty;
use cursive::{
    backends,
    utils::markup::StyledString,
    view::Nameable,
    views::{Canvas, Dialog, EditView, LinearLayout, TextView},
    Cursive, CursiveRunnable, CursiveRunner, View,
};

struct TTYLaunchData {
    tty_path: PathBuf,
    tty_file: File,
}

fn main() {
    let mut controller_siv = cursive::crossterm();

    controller_siv.add_layer(tty_launcher());

    controller_siv.run();

    let Some(mut tty_launch) = controller_siv.take_user_data::<TTYLaunchData>() else {
        eprintln!("Exit without setting tty launch data");
        return;
    };

    // for now
    println!(
        "yay! TODO: launching a tty at {}",
        tty_launch.tty_path.display()
    );
    writeln!(
        &mut tty_launch.tty_file,
        "hello, world! this is a remote crossterm!"
    )
    .unwrap();
}

fn tty_launcher() -> impl View {
    Dialog::new()
        .title("Select a /dev/tty")
        .padding_lrtb(1, 1, 0, 0)
        .content(
            LinearLayout::vertical()
                .child(TextView::new(
                    "Type the name of a tty.\n\
                     (Run `tty` in the target terminal to find out.))",
                ))
                .child(EditView::new().on_submit(|siv, content| {
                    // we have try blocks at home
                    let path = PathBuf::from(&content);
                    let res: io::Result<_> = (|| {
                        let file = fs::File::options().write(true).open(&path)?;
                        if !file.is_tty() {
                            Err(io::Error::other("didn't seem like a tty"))?;
                        }
                        Ok(file)
                    })();
                    match res {
                        Ok(tty_file) => {
                            siv.set_user_data(TTYLaunchData {
                                tty_file,
                                tty_path: path,
                            });
                            siv.quit();
                        }
                        Err(ono) => {
                            siv.add_layer(Dialog::info(format!(
                                "Could not use {} as a tty: {}",
                                content, ono
                            )));
                        }
                    }
                })),
        )
}
