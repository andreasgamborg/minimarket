pub mod app;/// Application.
pub mod event;/// Terminal events handler.
pub mod ui;/// Widget renderer.
pub mod tui;/// Terminal user interface.
pub mod update;
pub mod db;


use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use db::Database;
use update::update;

fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let database = Database::open("barcode.db")?;
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    database.save_product("5712876086464", "Majs")?;
    database.save_product("9788702165708", "Mimbo Jimbo og den lange vinter")?;
    database.save_product("5701101400229", "Makrel i tomat")?;
    database.save_product("5705550530032", "Torskerogn")?;
    database.save_product("5712871096444", "Flåede tomater")?;
    database.save_product("5712876129079", "Tomatpuré")?;
    database.save_product("5762385293914", "Leverpostej")?;
    database.save_product("5701018047432", "Tun i olie")?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {},
            Event::Key(key_event) => update(&mut app, &database, key_event),
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        };
    }

     // Exit the user interface.
    tui.exit()?;
    Ok(())
}
