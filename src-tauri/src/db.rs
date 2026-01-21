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
            photo_url TEXT,
            ai_summary TEXT,
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

        -- Diagnoses table
        CREATE TABLE IF NOT EXISTS diagnoses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            icd_code TEXT,
            onset_date TEXT,
            status TEXT DEFAULT 'active',
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Medications table
        CREATE TABLE IF NOT EXISTS medications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            dose TEXT,
            frequency TEXT,
            route TEXT,
            start_date TEXT,
            end_date TEXT,
            status TEXT DEFAULT 'active',
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Junction table for diagnosis-medication relationships
        CREATE TABLE IF NOT EXISTS diagnosis_medications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            diagnosis_id INTEGER NOT NULL,
            medication_id INTEGER NOT NULL,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (diagnosis_id) REFERENCES diagnoses(id) ON DELETE CASCADE,
            FOREIGN KEY (medication_id) REFERENCES medications(id) ON DELETE CASCADE,
            UNIQUE(diagnosis_id, medication_id)
        );

        -- Vitals table
        CREATE TABLE IF NOT EXISTS vitals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            vital_type TEXT NOT NULL,
            value REAL NOT NULL,
            value_secondary REAL,
            unit TEXT NOT NULL,
            recorded_at TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Labs table
        CREATE TABLE IF NOT EXISTS labs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            value REAL NOT NULL,
            unit TEXT,
            reference_range_low REAL,
            reference_range_high REAL,
            is_abnormal INTEGER DEFAULT 0,
            recorded_at TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Clinical scores table (PHQ-9, GAD-7, etc.)
        CREATE TABLE IF NOT EXISTS clinical_scores (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            score_type TEXT NOT NULL,
            score INTEGER NOT NULL,
            max_score INTEGER,
            interpretation TEXT,
            recorded_at TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Encounters table
        CREATE TABLE IF NOT EXISTS encounters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            encounter_date TEXT NOT NULL,
            encounter_type TEXT NOT NULL,
            chief_complaint TEXT,
            summary TEXT,
            note_content TEXT,
            provider TEXT,
            location TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Allergies table
        CREATE TABLE IF NOT EXISTS allergies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            allergen TEXT NOT NULL,
            reaction TEXT,
            severity TEXT,
            onset_date TEXT,
            status TEXT DEFAULT 'active',
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Vaccinations table
        CREATE TABLE IF NOT EXISTS vaccinations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            vaccine_name TEXT NOT NULL,
            date_given TEXT NOT NULL,
            lot_number TEXT,
            site TEXT,
            administered_by TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Social history table
        CREATE TABLE IF NOT EXISTS social_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            category TEXT NOT NULL,
            detail TEXT NOT NULL,
            status TEXT,
            start_date TEXT,
            end_date TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Family history table
        CREATE TABLE IF NOT EXISTS family_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            relation TEXT NOT NULL,
            condition TEXT NOT NULL,
            age_at_onset INTEGER,
            notes TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- To-dos table
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            diagnosis_id INTEGER,
            description TEXT NOT NULL,
            due_date TEXT,
            priority TEXT DEFAULT 'normal',
            status TEXT DEFAULT 'pending',
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE,
            FOREIGN KEY (diagnosis_id) REFERENCES diagnoses(id) ON DELETE SET NULL
        );

        -- Patient goals table
        CREATE TABLE IF NOT EXISTS goals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            target_date TEXT,
            status TEXT DEFAULT 'in_progress',
            progress INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Timeline events table
        CREATE TABLE IF NOT EXISTS timeline_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            event_type TEXT NOT NULL,
            description TEXT NOT NULL,
            event_date TEXT NOT NULL,
            icon TEXT,
            color TEXT,
            related_id INTEGER,
            related_table TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Indexes for performance
        CREATE INDEX IF NOT EXISTS idx_diagnoses_patient ON diagnoses(patient_id);
        CREATE INDEX IF NOT EXISTS idx_medications_patient ON medications(patient_id);
        CREATE INDEX IF NOT EXISTS idx_vitals_patient_date ON vitals(patient_id, recorded_at);
        CREATE INDEX IF NOT EXISTS idx_labs_patient_date ON labs(patient_id, recorded_at);
        CREATE INDEX IF NOT EXISTS idx_scores_patient_date ON clinical_scores(patient_id, recorded_at);
        CREATE INDEX IF NOT EXISTS idx_encounters_patient_date ON encounters(patient_id, encounter_date);
        CREATE INDEX IF NOT EXISTS idx_timeline_patient_date ON timeline_events(patient_id, event_date);
        "
    )?;

    // Add columns to existing patients table if they don't exist
    // SQLite doesn't have IF NOT EXISTS for ALTER TABLE, so we check first
    let has_photo_url: bool = conn
        .prepare("SELECT photo_url FROM patients LIMIT 1")
        .is_ok();
    if !has_photo_url {
        let _ = conn.execute("ALTER TABLE patients ADD COLUMN photo_url TEXT", []);
        let _ = conn.execute("ALTER TABLE patients ADD COLUMN ai_summary TEXT", []);
    }

    Ok(conn)
}

// ============ Patient CRUD Operations ============

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub photo_url: Option<String>,
    pub ai_summary: Option<String>,
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

// ============ New Structs for Patient Detail Page ============

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Diagnosis {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub name: String,
    pub icd_code: Option<String>,
    pub onset_date: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Medication {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub name: String,
    pub dose: Option<String>,
    pub frequency: Option<String>,
    pub route: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiagnosisWithMedications {
    pub diagnosis: Diagnosis,
    pub medication_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vital {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub vital_type: String,
    pub value: f64,
    pub value_secondary: Option<f64>,
    pub unit: String,
    pub recorded_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lab {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub name: String,
    pub value: f64,
    pub unit: Option<String>,
    pub reference_range_low: Option<f64>,
    pub reference_range_high: Option<f64>,
    pub is_abnormal: Option<bool>,
    pub recorded_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClinicalScore {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub score_type: String,
    pub score: i32,
    pub max_score: Option<i32>,
    pub interpretation: Option<String>,
    pub recorded_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Encounter {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub encounter_date: String,
    pub encounter_type: String,
    pub chief_complaint: Option<String>,
    pub summary: Option<String>,
    pub note_content: Option<String>,
    pub provider: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Allergy {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub allergen: String,
    pub reaction: Option<String>,
    pub severity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vaccination {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub vaccine_name: String,
    pub date_given: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SocialHistory {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub category: String,
    pub detail: String,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FamilyHistory {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub relation: String,
    pub condition: String,
    pub age_at_onset: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub diagnosis_id: Option<i64>,
    pub description: String,
    pub due_date: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Goal {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub description: String,
    pub target_date: Option<String>,
    pub status: Option<String>,
    pub progress: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimelineEvent {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub event_type: String,
    pub description: String,
    pub event_date: String,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientFullData {
    pub patient: Patient,
    pub diagnoses: Vec<DiagnosisWithMedications>,
    pub medications: Vec<Medication>,
    pub vitals: Vec<Vital>,
    pub labs: Vec<Lab>,
    pub clinical_scores: Vec<ClinicalScore>,
    pub encounters: Vec<Encounter>,
    pub allergies: Vec<Allergy>,
    pub vaccinations: Vec<Vaccination>,
    pub social_history: Vec<SocialHistory>,
    pub family_history: Vec<FamilyHistory>,
    pub todos: Vec<Todo>,
    pub goals: Vec<Goal>,
    pub timeline_events: Vec<TimelineEvent>,
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
        "SELECT id, first_name, last_name, dob, sex, gender, address, phone, email, photo_url, ai_summary FROM patients"
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
            photo_url: row.get(9)?,
            ai_summary: row.get(10)?,
        })
    })?;

    patients.collect()
}

pub fn get_patient_by_id(conn: &Connection, id: i64) -> Result<Option<Patient>> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, last_name, dob, sex, gender, address, phone, email, photo_url, ai_summary
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
            photo_url: row.get(9)?,
            ai_summary: row.get(10)?,
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

// ============ Patient Full Data (Aggregated) ============

pub fn get_patient_full_data(conn: &Connection, patient_id: i64) -> Result<Option<PatientFullData>> {
    // Get patient
    let patient = match get_patient_by_id(conn, patient_id)? {
        Some(p) => p,
        None => return Ok(None),
    };

    // Get diagnoses with their medication links
    let diagnoses = get_diagnoses_with_medications(conn, patient_id)?;

    // Get all medications
    let medications = get_medications_for_patient(conn, patient_id)?;

    // Get vitals
    let vitals = get_vitals_for_patient(conn, patient_id)?;

    // Get labs
    let labs = get_labs_for_patient(conn, patient_id)?;

    // Get clinical scores
    let clinical_scores = get_clinical_scores_for_patient(conn, patient_id)?;

    // Get encounters
    let encounters = get_encounters_for_patient(conn, patient_id)?;

    // Get allergies
    let allergies = get_allergies_for_patient(conn, patient_id)?;

    // Get vaccinations
    let vaccinations = get_vaccinations_for_patient(conn, patient_id)?;

    // Get social history
    let social_history = get_social_history_for_patient(conn, patient_id)?;

    // Get family history
    let family_history = get_family_history_for_patient(conn, patient_id)?;

    // Get todos
    let todos = get_todos_for_patient(conn, patient_id)?;

    // Get goals
    let goals = get_goals_for_patient(conn, patient_id)?;

    // Get timeline events
    let timeline_events = get_timeline_events_for_patient(conn, patient_id)?;

    Ok(Some(PatientFullData {
        patient,
        diagnoses,
        medications,
        vitals,
        labs,
        clinical_scores,
        encounters,
        allergies,
        vaccinations,
        social_history,
        family_history,
        todos,
        goals,
        timeline_events,
    }))
}

// ============ Diagnosis CRUD Operations ============

pub fn get_diagnoses_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Diagnosis>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, name, icd_code, onset_date, status
         FROM diagnoses WHERE patient_id = ?1 AND status = 'active'
         ORDER BY onset_date DESC"
    )?;

    let diagnoses = stmt.query_map(params![patient_id], |row| {
        Ok(Diagnosis {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            name: row.get(2)?,
            icd_code: row.get(3)?,
            onset_date: row.get(4)?,
            status: row.get(5)?,
        })
    })?;

    diagnoses.collect()
}

pub fn get_diagnoses_with_medications(conn: &Connection, patient_id: i64) -> Result<Vec<DiagnosisWithMedications>> {
    let diagnoses = get_diagnoses_for_patient(conn, patient_id)?;

    let mut result = Vec::new();
    for diagnosis in diagnoses {
        let diag_id = diagnosis.id.unwrap_or(0);
        let mut stmt = conn.prepare(
            "SELECT medication_id FROM diagnosis_medications WHERE diagnosis_id = ?1"
        )?;

        let med_ids: Vec<i64> = stmt.query_map(params![diag_id], |row| {
            row.get(0)
        })?.filter_map(|r| r.ok()).collect();

        result.push(DiagnosisWithMedications {
            diagnosis,
            medication_ids: med_ids,
        });
    }

    Ok(result)
}

// ============ Medication CRUD Operations ============

pub fn get_medications_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Medication>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, name, dose, frequency, route, start_date, end_date, status
         FROM medications WHERE patient_id = ?1 AND status = 'active'
         ORDER BY start_date DESC"
    )?;

    let medications = stmt.query_map(params![patient_id], |row| {
        Ok(Medication {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            name: row.get(2)?,
            dose: row.get(3)?,
            frequency: row.get(4)?,
            route: row.get(5)?,
            start_date: row.get(6)?,
            end_date: row.get(7)?,
            status: row.get(8)?,
        })
    })?;

    medications.collect()
}

// ============ Vitals CRUD Operations ============

pub fn get_vitals_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Vital>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, vital_type, value, value_secondary, unit, recorded_at
         FROM vitals WHERE patient_id = ?1
         ORDER BY recorded_at ASC"
    )?;

    let vitals = stmt.query_map(params![patient_id], |row| {
        Ok(Vital {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            vital_type: row.get(2)?,
            value: row.get(3)?,
            value_secondary: row.get(4)?,
            unit: row.get(5)?,
            recorded_at: row.get(6)?,
        })
    })?;

    vitals.collect()
}

// ============ Labs CRUD Operations ============

pub fn get_labs_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Lab>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, name, value, unit, reference_range_low, reference_range_high, is_abnormal, recorded_at
         FROM labs WHERE patient_id = ?1
         ORDER BY recorded_at ASC"
    )?;

    let labs = stmt.query_map(params![patient_id], |row| {
        let is_abnormal_int: Option<i32> = row.get(7)?;
        Ok(Lab {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            name: row.get(2)?,
            value: row.get(3)?,
            unit: row.get(4)?,
            reference_range_low: row.get(5)?,
            reference_range_high: row.get(6)?,
            is_abnormal: is_abnormal_int.map(|v| v != 0),
            recorded_at: row.get(8)?,
        })
    })?;

    labs.collect()
}

// ============ Clinical Scores CRUD Operations ============

pub fn get_clinical_scores_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<ClinicalScore>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, score_type, score, max_score, interpretation, recorded_at
         FROM clinical_scores WHERE patient_id = ?1
         ORDER BY recorded_at ASC"
    )?;

    let scores = stmt.query_map(params![patient_id], |row| {
        Ok(ClinicalScore {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            score_type: row.get(2)?,
            score: row.get(3)?,
            max_score: row.get(4)?,
            interpretation: row.get(5)?,
            recorded_at: row.get(6)?,
        })
    })?;

    scores.collect()
}

// ============ Encounters CRUD Operations ============

pub fn get_encounters_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Encounter>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, encounter_date, encounter_type, chief_complaint, summary, note_content, provider, location
         FROM encounters WHERE patient_id = ?1
         ORDER BY encounter_date DESC"
    )?;

    let encounters = stmt.query_map(params![patient_id], |row| {
        Ok(Encounter {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            encounter_date: row.get(2)?,
            encounter_type: row.get(3)?,
            chief_complaint: row.get(4)?,
            summary: row.get(5)?,
            note_content: row.get(6)?,
            provider: row.get(7)?,
            location: row.get(8)?,
        })
    })?;

    encounters.collect()
}

pub fn get_encounter_by_id(conn: &Connection, encounter_id: i64) -> Result<Option<Encounter>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, encounter_date, encounter_type, chief_complaint, summary, note_content, provider, location
         FROM encounters WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![encounter_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Encounter {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            encounter_date: row.get(2)?,
            encounter_type: row.get(3)?,
            chief_complaint: row.get(4)?,
            summary: row.get(5)?,
            note_content: row.get(6)?,
            provider: row.get(7)?,
            location: row.get(8)?,
        }))
    } else {
        Ok(None)
    }
}

// ============ Allergies CRUD Operations ============

pub fn get_allergies_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Allergy>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, allergen, reaction, severity
         FROM allergies WHERE patient_id = ?1 AND status = 'active'"
    )?;

    let allergies = stmt.query_map(params![patient_id], |row| {
        Ok(Allergy {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            allergen: row.get(2)?,
            reaction: row.get(3)?,
            severity: row.get(4)?,
        })
    })?;

    allergies.collect()
}

// ============ Vaccinations CRUD Operations ============

pub fn get_vaccinations_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Vaccination>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, vaccine_name, date_given
         FROM vaccinations WHERE patient_id = ?1
         ORDER BY date_given DESC"
    )?;

    let vaccinations = stmt.query_map(params![patient_id], |row| {
        Ok(Vaccination {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            vaccine_name: row.get(2)?,
            date_given: row.get(3)?,
        })
    })?;

    vaccinations.collect()
}

// ============ Social History CRUD Operations ============

pub fn get_social_history_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<SocialHistory>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, category, detail, status
         FROM social_history WHERE patient_id = ?1"
    )?;

    let history = stmt.query_map(params![patient_id], |row| {
        Ok(SocialHistory {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            category: row.get(2)?,
            detail: row.get(3)?,
            status: row.get(4)?,
        })
    })?;

    history.collect()
}

// ============ Family History CRUD Operations ============

pub fn get_family_history_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<FamilyHistory>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, relation, condition, age_at_onset
         FROM family_history WHERE patient_id = ?1"
    )?;

    let history = stmt.query_map(params![patient_id], |row| {
        Ok(FamilyHistory {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            relation: row.get(2)?,
            condition: row.get(3)?,
            age_at_onset: row.get(4)?,
        })
    })?;

    history.collect()
}

// ============ Todos CRUD Operations ============

pub fn get_todos_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Todo>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, diagnosis_id, description, due_date, priority, status
         FROM todos WHERE patient_id = ?1
         ORDER BY due_date ASC"
    )?;

    let todos = stmt.query_map(params![patient_id], |row| {
        Ok(Todo {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            diagnosis_id: row.get(2)?,
            description: row.get(3)?,
            due_date: row.get(4)?,
            priority: row.get(5)?,
            status: row.get(6)?,
        })
    })?;

    todos.collect()
}

// ============ Goals CRUD Operations ============

pub fn get_goals_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Goal>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, description, target_date, status, progress
         FROM goals WHERE patient_id = ?1
         ORDER BY target_date ASC"
    )?;

    let goals = stmt.query_map(params![patient_id], |row| {
        Ok(Goal {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            description: row.get(2)?,
            target_date: row.get(3)?,
            status: row.get(4)?,
            progress: row.get(5)?,
        })
    })?;

    goals.collect()
}

// ============ Timeline Events CRUD Operations ============

pub fn get_timeline_events_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<TimelineEvent>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, event_type, description, event_date, icon, color
         FROM timeline_events WHERE patient_id = ?1
         ORDER BY event_date ASC"
    )?;

    let events = stmt.query_map(params![patient_id], |row| {
        Ok(TimelineEvent {
            id: Some(row.get(0)?),
            patient_id: row.get(1)?,
            event_type: row.get(2)?,
            description: row.get(3)?,
            event_date: row.get(4)?,
            icon: row.get(5)?,
            color: row.get(6)?,
        })
    })?;

    events.collect()
}

// ============ Seed Patient Detail Test Data ============

pub fn seed_patient_detail_test_data(conn: &Connection, patient_id: i64) -> Result<()> {
    // Check if data already exists for this patient
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM diagnoses WHERE patient_id = ?1",
        params![patient_id],
        |row| row.get(0),
    )?;

    if count > 0 {
        return Ok(()); // Already seeded
    }

    // Update patient with AI summary
    conn.execute(
        "UPDATE patients SET ai_summary = ?1 WHERE id = ?2",
        params![
            "Middle-aged patient managing hypertension, heart failure, and depression. Responds well to beta-blocker therapy with improving cardiac function and PHQ-9 scores.",
            patient_id
        ],
    )?;

    // Seed diagnoses
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, onset_date, status)
         VALUES (?1, 'Hypertension', 'I10', '2020-03-15', 'active')",
        params![patient_id],
    )?;
    let htn_id = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, onset_date, status)
         VALUES (?1, 'Heart Failure with Reduced EF', 'I50.22', '2023-06-10', 'active')",
        params![patient_id],
    )?;
    let hfref_id = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, onset_date, status)
         VALUES (?1, 'Major Depressive Disorder', 'F33.0', '2019-08-20', 'active')",
        params![patient_id],
    )?;
    let depression_id = conn.last_insert_rowid();

    // Seed medications
    conn.execute(
        "INSERT INTO medications (patient_id, name, dose, frequency, route, start_date, status)
         VALUES (?1, 'Lisinopril', '10mg', 'daily', 'oral', '2020-03-15', 'active')",
        params![patient_id],
    )?;
    let lisinopril_id = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO medications (patient_id, name, dose, frequency, route, start_date, status)
         VALUES (?1, 'Carvedilol (Coreg)', '12.5mg', 'twice daily', 'oral', '2023-06-15', 'active')",
        params![patient_id],
    )?;
    let coreg_id = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO medications (patient_id, name, dose, frequency, route, start_date, status)
         VALUES (?1, 'Sertraline', '50mg', 'daily', 'oral', '2019-09-01', 'active')",
        params![patient_id],
    )?;
    let sertraline_id = conn.last_insert_rowid();

    // Link medications to diagnoses
    conn.execute(
        "INSERT INTO diagnosis_medications (diagnosis_id, medication_id) VALUES (?1, ?2)",
        params![htn_id, lisinopril_id],
    )?;

    // Coreg treats both HTN and HFrEF
    conn.execute(
        "INSERT INTO diagnosis_medications (diagnosis_id, medication_id) VALUES (?1, ?2)",
        params![htn_id, coreg_id],
    )?;

    conn.execute(
        "INSERT INTO diagnosis_medications (diagnosis_id, medication_id) VALUES (?1, ?2)",
        params![hfref_id, coreg_id],
    )?;

    conn.execute(
        "INSERT INTO diagnosis_medications (diagnosis_id, medication_id) VALUES (?1, ?2)",
        params![depression_id, sertraline_id],
    )?;

    // Seed vitals (BP readings over time with timestamps)
    let bp_readings = vec![
        ("2024-01-15T09:30:00", 142.0, 88.0),
        ("2024-02-10T10:15:00", 138.0, 85.0),
        ("2024-03-20T14:45:00", 135.0, 82.0),
        ("2024-04-15T11:00:00", 130.0, 80.0),
        ("2024-05-10T09:00:00", 128.0, 78.0),
    ];

    for (datetime, systolic, diastolic) in bp_readings {
        conn.execute(
            "INSERT INTO vitals (patient_id, vital_type, value, value_secondary, unit, recorded_at)
             VALUES (?1, 'BP', ?2, ?3, 'mmHg', ?4)",
            params![patient_id, systolic, diastolic, datetime],
        )?;
    }

    // Seed heart rate (with timestamps matching BP readings)
    let hr_readings = vec![
        ("2024-01-15T09:30:00", 82.0),
        ("2024-02-10T10:15:00", 78.0),
        ("2024-03-20T14:45:00", 75.0),
        ("2024-04-15T11:00:00", 72.0),
        ("2024-05-10T09:00:00", 70.0),
    ];

    for (datetime, hr) in hr_readings {
        conn.execute(
            "INSERT INTO vitals (patient_id, vital_type, value, unit, recorded_at)
             VALUES (?1, 'HR', ?2, 'bpm', ?3)",
            params![patient_id, hr, datetime],
        )?;
    }

    // Seed labs
    conn.execute(
        "INSERT INTO labs (patient_id, name, value, unit, reference_range_low, reference_range_high, is_abnormal, recorded_at)
         VALUES (?1, 'Glucose', 105.0, 'mg/dL', 70.0, 100.0, 1, '2024-05-10')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO labs (patient_id, name, value, unit, reference_range_low, reference_range_high, is_abnormal, recorded_at)
         VALUES (?1, 'Hemoglobin A1c', 5.8, '%', 4.0, 5.6, 1, '2024-05-10')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO labs (patient_id, name, value, unit, reference_range_low, reference_range_high, is_abnormal, recorded_at)
         VALUES (?1, 'Creatinine', 0.9, 'mg/dL', 0.7, 1.3, 0, '2024-05-10')",
        params![patient_id],
    )?;

    // Seed clinical scores (PHQ-9)
    let phq9_scores = vec![
        ("2024-01-15", 15, "Moderately severe"),
        ("2024-03-15", 12, "Moderate"),
        ("2024-05-15", 8, "Mild"),
    ];

    for (date, score, interpretation) in phq9_scores {
        conn.execute(
            "INSERT INTO clinical_scores (patient_id, score_type, score, max_score, interpretation, recorded_at)
             VALUES (?1, 'PHQ-9', ?2, 27, ?3, ?4)",
            params![patient_id, score, interpretation, date],
        )?;
    }

    // Seed encounters
    conn.execute(
        "INSERT INTO encounters (patient_id, encounter_date, encounter_type, chief_complaint, summary, note_content, provider, location)
         VALUES (?1, '2024-05-10', 'Office Visit', 'Routine follow-up', 'Patient doing well on current medications. BP improved.',
                 'SUBJECTIVE:\nPatient presents for routine follow-up of hypertension and depression.\nReports feeling well overall. Denies chest pain, shortness of breath.\nMood has been stable, sleeping well.\n\nOBJECTIVE:\nVitals: BP 128/78, HR 70, Temp 98.6\nGeneral: Well-appearing, NAD\nCardiac: RRR, no murmurs\nPsych: Mood euthymic, affect appropriate\n\nASSESSMENT/PLAN:\n1. Hypertension - well controlled on Lisinopril 10mg daily. Continue current regimen.\n2. MDD - improving on Sertraline 50mg daily. PHQ-9 down to 8. Continue current dose.\n\nReturn in 3 months or sooner if concerns.',
                 'Dr. Smith', 'Main Clinic')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO encounters (patient_id, encounter_date, encounter_type, chief_complaint, summary, note_content, provider, location)
         VALUES (?1, '2024-03-15', 'Office Visit', 'Depression follow-up', 'Mood improving on Sertraline.',
                 'SUBJECTIVE:\nPatient returns for depression follow-up.\nReports gradual improvement in mood over past 2 months.\nSleep improving, appetite normal.\n\nOBJECTIVE:\nPHQ-9: 12 (down from 15)\nPsych: Mood improved, less tearful\n\nASSESSMENT/PLAN:\n1. MDD - responding to Sertraline. Continue 50mg daily.\n\nFollow up in 2 months.',
                 'Dr. Smith', 'Main Clinic')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO encounters (patient_id, encounter_date, encounter_type, chief_complaint, summary, note_content, provider, location)
         VALUES (?1, '2024-01-15', 'Office Visit', 'New patient visit', 'Established care, initiated treatment for HTN and depression.',
                 'SUBJECTIVE:\nNew patient establishing care.\nHistory of hypertension diagnosed 4 years ago.\nDepression diagnosed 5 years ago, currently on Sertraline.\nReports fatigue and low mood despite medication.\n\nOBJECTIVE:\nVitals: BP 142/88, HR 82\nPHQ-9: 15 (moderately severe)\n\nASSESSMENT/PLAN:\n1. Hypertension - elevated today. Will continue Lisinopril and monitor.\n2. MDD - symptoms moderate despite Sertraline. Will continue current dose and reassess.\n\nLabs ordered: BMP, CBC, lipid panel, A1c\nReturn in 2 months.',
                 'Dr. Smith', 'Main Clinic')",
        params![patient_id],
    )?;

    // Seed allergies
    conn.execute(
        "INSERT INTO allergies (patient_id, allergen, reaction, severity, status)
         VALUES (?1, 'Penicillin', 'Rash', 'Moderate', 'active')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO allergies (patient_id, allergen, reaction, severity, status)
         VALUES (?1, 'Sulfa drugs', 'Hives', 'Mild', 'active')",
        params![patient_id],
    )?;

    // Seed vaccinations
    conn.execute(
        "INSERT INTO vaccinations (patient_id, vaccine_name, date_given)
         VALUES (?1, 'Influenza 2024-2025', '2024-10-15')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO vaccinations (patient_id, vaccine_name, date_given)
         VALUES (?1, 'COVID-19 Booster', '2024-09-01')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO vaccinations (patient_id, vaccine_name, date_given)
         VALUES (?1, 'Tdap', '2022-06-15')",
        params![patient_id],
    )?;

    // Seed social history
    conn.execute(
        "INSERT INTO social_history (patient_id, category, detail, status)
         VALUES (?1, 'Tobacco', 'Former smoker, quit 2018 (10 pack-year history)', 'Former')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO social_history (patient_id, category, detail, status)
         VALUES (?1, 'Alcohol', 'Social drinker, 2-3 drinks per week', 'Current')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO social_history (patient_id, category, detail, status)
         VALUES (?1, 'Exercise', 'Walks 30 minutes 3x/week', 'Current')",
        params![patient_id],
    )?;

    // Seed family history
    conn.execute(
        "INSERT INTO family_history (patient_id, relation, condition, age_at_onset)
         VALUES (?1, 'Father', 'Myocardial Infarction', 58)",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO family_history (patient_id, relation, condition, age_at_onset)
         VALUES (?1, 'Mother', 'Type 2 Diabetes', 62)",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO family_history (patient_id, relation, condition, age_at_onset)
         VALUES (?1, 'Sister', 'Depression', 35)",
        params![patient_id],
    )?;

    // Seed todos
    conn.execute(
        "INSERT INTO todos (patient_id, diagnosis_id, description, due_date, priority, status)
         VALUES (?1, ?2, 'Check potassium level (on ACE inhibitor)', '2024-06-15', 'normal', 'pending')",
        params![patient_id, htn_id],
    )?;

    conn.execute(
        "INSERT INTO todos (patient_id, diagnosis_id, description, due_date, priority, status)
         VALUES (?1, ?2, 'Consider dose increase if PHQ-9 plateaus', '2024-07-15', 'low', 'pending')",
        params![patient_id, depression_id],
    )?;

    conn.execute(
        "INSERT INTO todos (patient_id, description, due_date, priority, status)
         VALUES (?1, 'Schedule colonoscopy (due for screening)', '2024-08-01', 'normal', 'pending')",
        params![patient_id],
    )?;

    // Seed goals
    conn.execute(
        "INSERT INTO goals (patient_id, description, target_date, status, progress)
         VALUES (?1, 'Achieve BP < 130/80 consistently', '2024-12-31', 'in_progress', 80)",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO goals (patient_id, description, target_date, status, progress)
         VALUES (?1, 'Reduce PHQ-9 score to < 5', '2024-12-31', 'in_progress', 60)",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO goals (patient_id, description, target_date, status, progress)
         VALUES (?1, 'Increase exercise to 5x/week', '2024-09-01', 'in_progress', 40)",
        params![patient_id],
    )?;

    // Seed timeline events
    conn.execute(
        "INSERT INTO timeline_events (patient_id, event_type, description, event_date, icon, color)
         VALUES (?1, 'diagnosis', 'Diagnosed with Major Depressive Disorder', '2019-08-20', 'fa-brain', 'purple')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO timeline_events (patient_id, event_type, description, event_date, icon, color)
         VALUES (?1, 'medication_start', 'Started Sertraline 50mg', '2019-09-01', 'fa-pills', 'green')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO timeline_events (patient_id, event_type, description, event_date, icon, color)
         VALUES (?1, 'diagnosis', 'Diagnosed with Hypertension', '2020-03-15', 'fa-heart-pulse', 'red')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO timeline_events (patient_id, event_type, description, event_date, icon, color)
         VALUES (?1, 'medication_start', 'Started Lisinopril 10mg', '2020-03-15', 'fa-pills', 'green')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO timeline_events (patient_id, event_type, description, event_date, icon, color)
         VALUES (?1, 'diagnosis', 'Diagnosed with Heart Failure (HFrEF)', '2023-06-10', 'fa-heart-crack', 'red')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO timeline_events (patient_id, event_type, description, event_date, icon, color)
         VALUES (?1, 'medication_start', 'Started Carvedilol (Coreg) 12.5mg BID', '2023-06-15', 'fa-pills', 'green')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO timeline_events (patient_id, event_type, description, event_date, icon, color)
         VALUES (?1, 'encounter', 'Established care with Dr. Smith', '2024-01-15', 'fa-user-doctor', 'blue')",
        params![patient_id],
    )?;

    conn.execute(
        "INSERT INTO timeline_events (patient_id, event_type, description, event_date, icon, color)
         VALUES (?1, 'lab_result', 'A1c slightly elevated at 5.8%', '2024-05-10', 'fa-flask', 'orange')",
        params![patient_id],
    )?;

    Ok(())
}
