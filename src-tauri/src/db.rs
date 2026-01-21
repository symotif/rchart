use rusqlite::{Connection, Result, params};
use std::sync::Mutex;
use tauri::AppHandle;
use std::path::PathBuf;

/// Get the database path in the app's data directory
pub fn get_db_path(app_handle: &AppHandle) -> PathBuf {
    let app_dir = app_handle
        .path_resolver()
        .app_dir()
        .expect("Failed to get app data dir");

    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");

    app_dir.join("rchart.db")
}

/// Database state that will be managed by Tauri
pub struct DbState(pub Mutex<Connection>);

/// Initialize the database with SQLCipher encryption and create tables if they don't exist
///
/// IMPORTANT for HIPAA compliance:
/// - The encryption key should NOT be hardcoded in production
/// - Use a secure key derivation from user credentials or secure storage
/// - Consider using the OS keychain (keyring crate) to store the key
pub fn init_db(db_path: &PathBuf, _encryption_key: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // TODO: Re-enable encryption for production!
    // SQLCipher encryption is temporarily disabled for development.
    // Uncomment the lines below to enable encryption:
    //
    // conn.pragma_update(None, "key", encryption_key)?;
    // conn.pragma_update(None, "cipher_page_size", 4096)?;
    // conn.pragma_update(None, "kdf_iter", 256000)?;
    // conn.pragma_update(None, "cipher_hmac_algorithm", "HMAC_SHA512")?;
    // conn.pragma_update(None, "cipher_kdf_algorithm", "PBKDF2_HMAC_SHA512")?;

    // Create tables
    conn.execute_batch(
        "
        -- Patients table
        CREATE TABLE IF NOT EXISTS patients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            dob TEXT NOT NULL,
            sex TEXT NOT NULL,
            gender TEXT,
            address TEXT,
            phone TEXT,
            email TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT DEFAULT (datetime('now', 'localtime'))
        );

        -- Appointments table
        CREATE TABLE IF NOT EXISTS appointments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            appointment_time TEXT NOT NULL,
            duration_minutes INTEGER DEFAULT 30,
            reason TEXT,
            status TEXT DEFAULT 'scheduled',
            notes TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id)
        );

        -- Messages table
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER,
            subject TEXT NOT NULL,
            body TEXT NOT NULL,
            is_read INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id)
        );
        "
    )?;

    Ok(conn)
}

// ============ Patient CRUD Operations ============

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Patient {
    pub id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    pub sex: String,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Appointment {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub appointment_time: String,
    pub duration_minutes: Option<i32>,
    pub reason: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentWithPatient {
    pub id: i64,
    pub patient_id: i64,
    pub patient_name: String,
    pub appointment_time: String,
    pub duration_minutes: i32,
    pub reason: Option<String>,
    pub status: String,
}

pub fn create_patient(conn: &Connection, patient: &Patient) -> Result<i64> {
    conn.execute(
        "INSERT INTO patients (first_name, last_name, dob, sex, gender, address, phone, email)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            patient.first_name,
            patient.last_name,
            patient.dob,
            patient.sex,
            patient.gender,
            patient.address,
            patient.phone,
            patient.email,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_all_patients(conn: &Connection) -> Result<Vec<Patient>> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, last_name, dob, sex, gender, address, phone, email FROM patients"
    )?;

    let patients = stmt.query_map([], |row| {
        Ok(Patient {
            id: Some(row.get(0)?),
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            dob: row.get(3)?,
            sex: row.get(4)?,
            gender: row.get(5)?,
            address: row.get(6)?,
            phone: row.get(7)?,
            email: row.get(8)?,
        })
    })?;

    patients.collect()
}

pub fn get_patient_by_id(conn: &Connection, id: i64) -> Result<Option<Patient>> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, last_name, dob, sex, gender, address, phone, email
         FROM patients WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Patient {
            id: Some(row.get(0)?),
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            dob: row.get(3)?,
            sex: row.get(4)?,
            gender: row.get(5)?,
            address: row.get(6)?,
            phone: row.get(7)?,
            email: row.get(8)?,
        }))
    } else {
        Ok(None)
    }
}

// ============ Appointment CRUD Operations ============

pub fn create_appointment(conn: &Connection, appointment: &Appointment) -> Result<i64> {
    conn.execute(
        "INSERT INTO appointments (patient_id, appointment_time, duration_minutes, reason, status, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            appointment.patient_id,
            appointment.appointment_time,
            appointment.duration_minutes.unwrap_or(30),
            appointment.reason,
            appointment.status.as_deref().unwrap_or("scheduled"),
            appointment.notes,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_appointments_for_date(conn: &Connection, date: &str) -> Result<Vec<AppointmentWithPatient>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.patient_id, p.first_name || ' ' || p.last_name,
                a.appointment_time, a.duration_minutes, a.reason, a.status
         FROM appointments a
         JOIN patients p ON a.patient_id = p.id
         WHERE date(a.appointment_time) = date(?1)
         ORDER BY a.appointment_time"
    )?;

    let appointments = stmt.query_map(params![date], |row| {
        Ok(AppointmentWithPatient {
            id: row.get(0)?,
            patient_id: row.get(1)?,
            patient_name: row.get(2)?,
            appointment_time: row.get(3)?,
            duration_minutes: row.get(4)?,
            reason: row.get(5)?,
            status: row.get(6)?,
        })
    })?;

    appointments.collect()
}

pub fn get_all_appointments(conn: &Connection) -> Result<Vec<AppointmentWithPatient>> {
    let mut stmt = conn.prepare(
        "SELECT a.id, a.patient_id, p.first_name || ' ' || p.last_name,
                a.appointment_time, a.duration_minutes, a.reason, a.status
         FROM appointments a
         JOIN patients p ON a.patient_id = p.id
         ORDER BY a.appointment_time"
    )?;

    let appointments = stmt.query_map([], |row| {
        Ok(AppointmentWithPatient {
            id: row.get(0)?,
            patient_id: row.get(1)?,
            patient_name: row.get(2)?,
            appointment_time: row.get(3)?,
            duration_minutes: row.get(4)?,
            reason: row.get(5)?,
            status: row.get(6)?,
        })
    })?;

    appointments.collect()
}
