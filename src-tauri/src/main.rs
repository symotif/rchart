// disable the command prompt window that would normally pop up if someone is on windows running a bundled app
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod db;

use db::{DbState, Patient, Appointment, AppointmentWithPatient, PatientFullData, Encounter, User, UserFullData, UserSettings};
use serde::{Deserialize, Serialize};
use tauri::{State, Manager};
use std::sync::Mutex;

// ============ Legacy Appointment struct (for backwards compatibility) ============
#[derive(Serialize, Deserialize)]
pub struct LegacyAppointment {
    pub name: String,
    pub age: u64,
    pub sex: String,
    pub time: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Get the database path
            let db_path = db::get_db_path(&app.handle());
            println!("Database path: {:?}", db_path);

            // IMPORTANT: In production, this key should come from:
            // - User authentication (derived from password)
            // - OS keychain/secure storage
            // - Environment variable (for development only)
            // DO NOT hardcode in production!
            let encryption_key = std::env::var("RCHART_DB_KEY")
                .unwrap_or_else(|_| "CHANGE_THIS_KEY_IN_PRODUCTION".to_string());

            // Initialize the encrypted database
            let conn = db::init_db(&db_path, &encryption_key)
                .expect("Failed to initialize database");

            // Manage the database connection state
            app.manage(DbState(Mutex::new(conn)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Legacy commands
            get_month,
            greet,
            get_appointments,
            // New database commands
            db_create_patient,
            db_get_all_patients,
            db_get_patient,
            db_create_appointment,
            db_get_appointments_for_date,
            db_get_all_appointments,
            db_seed_test_data,
            // Patient detail page commands
            db_get_patient_full,
            db_get_encounter,
            db_seed_patient_detail_test_data,
            // Encounter CRUD commands
            db_create_encounter,
            db_update_encounter,
            // User/Provider commands
            db_get_current_user,
            db_get_user_full,
            db_update_user,
            db_update_user_settings,
            db_update_password,
            db_seed_user_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ============ Legacy Commands (kept for backwards compatibility) ============

#[tauri::command]
async fn get_month() -> String {
    "May".to_string()
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You have been greeted from Rust!", name)
}

#[tauri::command]
fn get_appointments() -> Vec<LegacyAppointment> {
    vec![
        LegacyAppointment {
            name: String::from("Logan"),
            age: 24,
            sex: String::from("Male"),
            time: String::from("3:30"),
        },
        LegacyAppointment {
            name: String::from("Clarance"),
            age: 25,
            sex: String::from("Male"),
            time: String::from("4:30"),
        },
        LegacyAppointment {
            name: String::from("Tristy"),
            age: 44,
            sex: String::from("Female"),
            time: String::from("5:30"),
        },
    ]
}

// ============ New Database Commands ============

#[tauri::command]
fn db_create_patient(state: State<DbState>, patient: Patient) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_patient(&conn, &patient).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_get_all_patients(state: State<DbState>) -> Result<Vec<Patient>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_all_patients(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_get_patient(state: State<DbState>, id: i64) -> Result<Option<Patient>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_patient_by_id(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_create_appointment(state: State<DbState>, appointment: Appointment) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_appointment(&conn, &appointment).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_get_appointments_for_date(state: State<DbState>, date: String) -> Result<Vec<AppointmentWithPatient>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_appointments_for_date(&conn, &date).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_get_all_appointments(state: State<DbState>) -> Result<Vec<AppointmentWithPatient>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_all_appointments(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_seed_test_data(state: State<DbState>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // Check if we already have patients
    let existing = db::get_all_patients(&conn).map_err(|e| e.to_string())?;
    if !existing.is_empty() {
        return Ok(format!("Database already has {} patients", existing.len()));
    }

    // Seed test patients
    let test_patients = vec![
        Patient {
            id: None,
            first_name: "Logan".to_string(),
            last_name: "Nguyen".to_string(),
            dob: "1998-09-15".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("222 Main St. Detroit".to_string()),
            phone: Some("555-555-5555".to_string()),
            email: Some("logan@example.com".to_string()),
            photo_url: None,
            ai_summary: None,
            preferred_pharmacy: Some("Walgreens - 100 Main St, Detroit".to_string()),
            insurance_provider: Some("Aetna".to_string()),
            insurance_policy_number: Some("AET123456".to_string()),
            insurance_group_number: Some("GRP100".to_string()),
        },
        Patient {
            id: None,
            first_name: "Sarah".to_string(),
            last_name: "Johnson".to_string(),
            dob: "1985-03-22".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("456 Oak Ave. Chicago".to_string()),
            phone: Some("555-123-4567".to_string()),
            email: Some("sarah.j@example.com".to_string()),
            photo_url: None,
            ai_summary: None,
            preferred_pharmacy: Some("CVS Pharmacy - 200 Oak Ave, Chicago".to_string()),
            insurance_provider: Some("United Healthcare".to_string()),
            insurance_policy_number: Some("UHC789012".to_string()),
            insurance_group_number: Some("GRP200".to_string()),
        },
        Patient {
            id: None,
            first_name: "Michael".to_string(),
            last_name: "Chen".to_string(),
            dob: "1990-07-10".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("789 Pine Rd. Seattle".to_string()),
            phone: Some("555-987-6543".to_string()),
            email: Some("m.chen@example.com".to_string()),
            photo_url: None,
            ai_summary: None,
            preferred_pharmacy: Some("Rite Aid - 300 Pine Rd, Seattle".to_string()),
            insurance_provider: Some("Cigna".to_string()),
            insurance_policy_number: Some("CIG345678".to_string()),
            insurance_group_number: Some("GRP300".to_string()),
        },
        Patient {
            id: None,
            first_name: "Emily".to_string(),
            last_name: "Davis".to_string(),
            dob: "1978-12-01".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("321 Elm St. Boston".to_string()),
            phone: Some("555-456-7890".to_string()),
            email: Some("emily.d@example.com".to_string()),
            photo_url: None,
            ai_summary: None,
            preferred_pharmacy: None,
            insurance_provider: Some("Blue Cross Blue Shield".to_string()),
            insurance_policy_number: Some("BCBS901234".to_string()),
            insurance_group_number: Some("GRP400".to_string()),
        },
        Patient {
            id: None,
            first_name: "James".to_string(),
            last_name: "Wilson".to_string(),
            dob: "2000-05-18".to_string(),
            sex: "M".to_string(),
            gender: Some("NB".to_string()),
            address: Some("654 Maple Dr. Austin".to_string()),
            phone: Some("555-321-0987".to_string()),
            email: Some("j.wilson@example.com".to_string()),
            photo_url: None,
            ai_summary: None,
            preferred_pharmacy: Some("Costco Pharmacy - 500 Maple Dr, Austin".to_string()),
            insurance_provider: None,
            insurance_policy_number: None,
            insurance_group_number: None,
        },
    ];

    let mut created = 0;
    for patient in &test_patients {
        db::create_patient(&conn, patient).map_err(|e| e.to_string())?;
        created += 1;
    }

    Ok(format!("Seeded {} test patients", created))
}

// ============ Patient Detail Page Commands ============

#[tauri::command]
fn db_get_patient_full(state: State<DbState>, id: i64) -> Result<Option<PatientFullData>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_patient_full_data(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_get_encounter(state: State<DbState>, encounter_id: i64) -> Result<Option<Encounter>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_encounter_by_id(&conn, encounter_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_seed_patient_detail_test_data(state: State<DbState>, patient_id: i64, force_reseed: Option<bool>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::seed_patient_detail_test_data(&conn, patient_id, force_reseed.unwrap_or(false)).map_err(|e| e.to_string())?;
    Ok(format!("Seeded detail data for patient {}", patient_id))
}

#[tauri::command]
fn db_create_encounter(state: State<DbState>, encounter: Encounter) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_encounter(&conn, &encounter).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_encounter(state: State<DbState>, encounter: Encounter) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_encounter(&conn, &encounter).map_err(|e| e.to_string())
}

// ============ User/Provider Commands ============

#[tauri::command]
fn db_get_current_user(state: State<DbState>) -> Result<Option<UserFullData>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_current_user_full_data(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_get_user_full(state: State<DbState>, id: i64) -> Result<Option<UserFullData>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_user_full_data(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_user(state: State<DbState>, user: User) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_user(&conn, &user).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_user_settings(state: State<DbState>, settings: UserSettings) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_user_settings(&conn, &settings).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_password(state: State<DbState>, user_id: i64, new_password_hash: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_user_password(&conn, user_id, &new_password_hash).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_seed_user_data(state: State<DbState>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::seed_user_data(&conn).map_err(|e| e.to_string())?;
    Ok("User data seeded successfully".to_string())
}
