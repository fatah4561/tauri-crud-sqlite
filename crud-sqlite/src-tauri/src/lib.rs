// Import functionalities we'll be using
pub mod models;
pub mod configs;
pub mod migrations;
pub mod repositories;
pub mod services;
pub mod seeders;

use chrono;
use std::process;
use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Manager, State};
use tokio::time::{sleep, Duration};

// traits import
use repositories::ProductRepository;

// Create a struct we'll use to track the completion of
// setup related tasks
struct SetupState {
    frontend_task: bool,
    backend_task: bool,
}

// Our main entrypoint in a version 2 mobile compatible app
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Don't write code before Tauri starts, write it in the
    // setup hook instead!
    tauri::Builder::default()
        // Register a `State` to be managed by Tauri
        // We need write access to it so we wrap it in a `Mutex`
        .manage(Mutex::new(SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        // Add a command we can use to check
        .invoke_handler(tauri::generate_handler![greet, set_complete])
        // Use the setup hook to execute setup related tasks
        // Runs before the main loop, so no windows are yet created
        .setup(|app| {
            // Spawn setup as a non-blocking task so the windows can be
            // created and ran while it executes
            spawn(setup(app.handle().clone()));
            // The hook expects an Ok result
            Ok(())
        })
        // Run the app
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello {name} from Rust!")
}

// A custom task for setting the state of a setup task
#[tauri::command]
async fn set_complete(
    app: AppHandle,
    state: State<'_, Mutex<SetupState>>,
    task: String,
) -> Result<(), ()> {
    // Lock the state without write access
    let mut state_lock = state.lock().unwrap();
    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        _ => panic!("invalid task completed!"),
    }
    // Check if both tasks are completed
    if state_lock.backend_task && state_lock.frontend_task {
        // Setup is complete, we can close the splashscreen
        // and unhide the main window!
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
    Ok(())
}

// An async function that does some heavy setup task
async fn setup(app: AppHandle) -> Result<(), ()> {
    
    println!("Performing really heavy backend setup task...");
    let conn = configs::new_sqlite_db().unwrap_or_else(|err| {
        eprintln!("Error creating db connection: {err}");
        process::exit(1)
    }); 

    migrations::migrate(&conn).unwrap_or_else(|err| {
        eprintln!("Error migrating db: {err}");
        process::exit(1)
    });

    let product = models::Product { 
        id: 0,
        name: "test".to_string(), 
        base_price: 0, 
        created_at: chrono::Utc::now(), 
        updated_at: chrono::Utc::now()
    };

    match conn.insert_product(product) {
        Ok(_) => println!("Product created successfully"),
        Err(e) => eprintln!("Error creating product: {}", e),
    }

    let products = conn.get_all_products();

    match products {
        Ok(products) => {
            for product in products {
                println!("Product: {:?}", product);
            }
        },
        Err(e) => eprintln!("Error getting products: {}", e),
    }

    // Fake performing some heavy action for 3 seconds
    // sqlite::run_sqlite();
    sleep(Duration::from_secs(3)).await;
    println!("Backend setup task completed!");
    // Set the backend task as being completed
    // Commands can be ran as regular functions as long as you take
    // care of the input arguments yourself
    set_complete(
        app.clone(),
        app.state::<Mutex<SetupState>>(),
        "backend".to_string(),
    )
    .await?;
    Ok(())
}