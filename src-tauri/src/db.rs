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
            preferred_pharmacy TEXT,
            insurance_provider TEXT,
            insurance_policy_number TEXT,
            insurance_group_number TEXT,
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
            description TEXT,
            icd_code TEXT,
            onset_date TEXT,
            status TEXT DEFAULT 'active',
            category TEXT,
            notes TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );

        -- Medications table
        CREATE TABLE IF NOT EXISTS medications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            dose TEXT,
            dosage TEXT,
            frequency TEXT,
            route TEXT,
            prescriber TEXT,
            start_date TEXT,
            end_date TEXT,
            status TEXT DEFAULT 'active',
            notes TEXT,
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
            test_name TEXT,
            value REAL NOT NULL,
            result TEXT,
            unit TEXT,
            reference_range_low REAL,
            reference_range_high REAL,
            is_abnormal INTEGER DEFAULT 0,
            notes TEXT,
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

        -- Users/Providers table
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            degree_type TEXT,
            specialty TEXT,
            subspecialty TEXT,
            npi_number TEXT,
            photo_url TEXT,
            bio TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT DEFAULT (datetime('now', 'localtime'))
        );

        -- User education/training table
        CREATE TABLE IF NOT EXISTS user_education (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            education_type TEXT NOT NULL,
            institution TEXT NOT NULL,
            degree TEXT,
            field_of_study TEXT,
            start_year INTEGER,
            end_year INTEGER,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );

        -- User badges/awards table
        CREATE TABLE IF NOT EXISTS user_badges (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            badge_name TEXT NOT NULL,
            badge_type TEXT NOT NULL,
            description TEXT,
            icon TEXT,
            color TEXT,
            awarded_date TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );

        -- User settings table
        CREATE TABLE IF NOT EXISTS user_settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL UNIQUE,
            language TEXT DEFAULT 'en',
            notifications_enabled INTEGER DEFAULT 1,
            email_notifications INTEGER DEFAULT 1,
            sms_notifications INTEGER DEFAULT 0,
            two_factor_enabled INTEGER DEFAULT 0,
            two_factor_secret TEXT,
            zen_mode_default INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );

        -- Patient lists (custom patient groupings per user)
        CREATE TABLE IF NOT EXISTS patient_lists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            color TEXT DEFAULT '#3B82F6',
            icon TEXT DEFAULT 'fa-list',
            is_default INTEGER DEFAULT 0,
            sort_order INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );

        -- Patient list members (join table)
        CREATE TABLE IF NOT EXISTS patient_list_members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            list_id INTEGER NOT NULL,
            patient_id INTEGER NOT NULL,
            added_at TEXT DEFAULT (datetime('now', 'localtime')),
            notes TEXT,
            FOREIGN KEY (list_id) REFERENCES patient_lists(id) ON DELETE CASCADE,
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE,
            UNIQUE(list_id, patient_id)
        );

        -- Patient list columns (custom columns for each list)
        CREATE TABLE IF NOT EXISTS patient_list_columns (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            list_id INTEGER NOT NULL,
            column_key TEXT NOT NULL,
            column_label TEXT NOT NULL,
            column_type TEXT DEFAULT 'text',
            is_visible INTEGER DEFAULT 1,
            sort_order INTEGER DEFAULT 0,
            width INTEGER DEFAULT 150,
            FOREIGN KEY (list_id) REFERENCES patient_lists(id) ON DELETE CASCADE
        );

        -- Prescriptions table (tracks prescription history)
        CREATE TABLE IF NOT EXISTS prescriptions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            medication_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            days_supply INTEGER NOT NULL,
            refills INTEGER DEFAULT 0,
            sig TEXT NOT NULL,
            pharmacy TEXT,
            prescriber_id INTEGER,
            status TEXT DEFAULT 'sent',
            prescribed_date TEXT DEFAULT (datetime('now', 'localtime')),
            filled_date TEXT,
            notes TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE,
            FOREIGN KEY (medication_id) REFERENCES medications(id) ON DELETE CASCADE,
            FOREIGN KEY (prescriber_id) REFERENCES users(id) ON DELETE SET NULL
        );

        -- Indexes for performance
        CREATE INDEX IF NOT EXISTS idx_prescriptions_patient ON prescriptions(patient_id);
        CREATE INDEX IF NOT EXISTS idx_prescriptions_medication ON prescriptions(medication_id);
        CREATE INDEX IF NOT EXISTS idx_diagnoses_patient ON diagnoses(patient_id);
        CREATE INDEX IF NOT EXISTS idx_medications_patient ON medications(patient_id);
        CREATE INDEX IF NOT EXISTS idx_vitals_patient_date ON vitals(patient_id, recorded_at);
        CREATE INDEX IF NOT EXISTS idx_labs_patient_date ON labs(patient_id, recorded_at);
        CREATE INDEX IF NOT EXISTS idx_scores_patient_date ON clinical_scores(patient_id, recorded_at);
        CREATE INDEX IF NOT EXISTS idx_encounters_patient_date ON encounters(patient_id, encounter_date);
        CREATE INDEX IF NOT EXISTS idx_timeline_patient_date ON timeline_events(patient_id, event_date);
        CREATE INDEX IF NOT EXISTS idx_user_education_user ON user_education(user_id);
        CREATE INDEX IF NOT EXISTS idx_user_badges_user ON user_badges(user_id);
        CREATE INDEX IF NOT EXISTS idx_patient_lists_user ON patient_lists(user_id);
        CREATE INDEX IF NOT EXISTS idx_patient_list_members_list ON patient_list_members(list_id);
        CREATE INDEX IF NOT EXISTS idx_patient_list_columns_list ON patient_list_columns(list_id);

        -- FTS5 Full-Text Search virtual tables for fast global search
        CREATE VIRTUAL TABLE IF NOT EXISTS patients_fts USING fts5(
            patient_id UNINDEXED,
            first_name,
            last_name,
            phone,
            email,
            address,
            ai_summary,
            content='patients',
            content_rowid='id'
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS encounters_fts USING fts5(
            encounter_id UNINDEXED,
            patient_id UNINDEXED,
            encounter_type,
            chief_complaint,
            summary,
            note_content,
            provider,
            content='encounters',
            content_rowid='id'
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS diagnoses_fts USING fts5(
            diagnosis_id UNINDEXED,
            patient_id UNINDEXED,
            icd_code,
            description,
            category,
            notes,
            content='diagnoses',
            content_rowid='id'
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS medications_fts USING fts5(
            medication_id UNINDEXED,
            patient_id UNINDEXED,
            name,
            dosage,
            frequency,
            prescriber,
            notes,
            content='medications',
            content_rowid='id'
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS labs_fts USING fts5(
            lab_id UNINDEXED,
            patient_id UNINDEXED,
            test_name,
            result,
            unit,
            notes,
            content='labs',
            content_rowid='id'
        );

        -- Triggers to keep FTS tables in sync with source tables
        CREATE TRIGGER IF NOT EXISTS patients_ai AFTER INSERT ON patients BEGIN
            INSERT INTO patients_fts(rowid, patient_id, first_name, last_name, phone, email, address, ai_summary)
            VALUES (new.id, new.id, new.first_name, new.last_name, new.phone, new.email, new.address, new.ai_summary);
        END;

        CREATE TRIGGER IF NOT EXISTS patients_ad AFTER DELETE ON patients BEGIN
            INSERT INTO patients_fts(patients_fts, rowid, patient_id, first_name, last_name, phone, email, address, ai_summary)
            VALUES ('delete', old.id, old.id, old.first_name, old.last_name, old.phone, old.email, old.address, old.ai_summary);
        END;

        CREATE TRIGGER IF NOT EXISTS patients_au AFTER UPDATE ON patients BEGIN
            INSERT INTO patients_fts(patients_fts, rowid, patient_id, first_name, last_name, phone, email, address, ai_summary)
            VALUES ('delete', old.id, old.id, old.first_name, old.last_name, old.phone, old.email, old.address, old.ai_summary);
            INSERT INTO patients_fts(rowid, patient_id, first_name, last_name, phone, email, address, ai_summary)
            VALUES (new.id, new.id, new.first_name, new.last_name, new.phone, new.email, new.address, new.ai_summary);
        END;

        CREATE TRIGGER IF NOT EXISTS encounters_ai AFTER INSERT ON encounters BEGIN
            INSERT INTO encounters_fts(rowid, encounter_id, patient_id, encounter_type, chief_complaint, summary, note_content, provider)
            VALUES (new.id, new.id, new.patient_id, new.encounter_type, new.chief_complaint, new.summary, new.note_content, new.provider);
        END;

        CREATE TRIGGER IF NOT EXISTS encounters_ad AFTER DELETE ON encounters BEGIN
            INSERT INTO encounters_fts(encounters_fts, rowid, encounter_id, patient_id, encounter_type, chief_complaint, summary, note_content, provider)
            VALUES ('delete', old.id, old.id, old.patient_id, old.encounter_type, old.chief_complaint, old.summary, old.note_content, old.provider);
        END;

        CREATE TRIGGER IF NOT EXISTS encounters_au AFTER UPDATE ON encounters BEGIN
            INSERT INTO encounters_fts(encounters_fts, rowid, encounter_id, patient_id, encounter_type, chief_complaint, summary, note_content, provider)
            VALUES ('delete', old.id, old.id, old.patient_id, old.encounter_type, old.chief_complaint, old.summary, old.note_content, old.provider);
            INSERT INTO encounters_fts(rowid, encounter_id, patient_id, encounter_type, chief_complaint, summary, note_content, provider)
            VALUES (new.id, new.id, new.patient_id, new.encounter_type, new.chief_complaint, new.summary, new.note_content, new.provider);
        END;

        CREATE TRIGGER IF NOT EXISTS diagnoses_ai AFTER INSERT ON diagnoses BEGIN
            INSERT INTO diagnoses_fts(rowid, diagnosis_id, patient_id, icd_code, description, category, notes)
            VALUES (new.id, new.id, new.patient_id, new.icd_code, new.description, new.category, new.notes);
        END;

        CREATE TRIGGER IF NOT EXISTS diagnoses_ad AFTER DELETE ON diagnoses BEGIN
            INSERT INTO diagnoses_fts(diagnoses_fts, rowid, diagnosis_id, patient_id, icd_code, description, category, notes)
            VALUES ('delete', old.id, old.id, old.patient_id, old.icd_code, old.description, old.category, old.notes);
        END;

        CREATE TRIGGER IF NOT EXISTS diagnoses_au AFTER UPDATE ON diagnoses BEGIN
            INSERT INTO diagnoses_fts(diagnoses_fts, rowid, diagnosis_id, patient_id, icd_code, description, category, notes)
            VALUES ('delete', old.id, old.id, old.patient_id, old.icd_code, old.description, old.category, old.notes);
            INSERT INTO diagnoses_fts(rowid, diagnosis_id, patient_id, icd_code, description, category, notes)
            VALUES (new.id, new.id, new.patient_id, new.icd_code, new.description, new.category, new.notes);
        END;

        CREATE TRIGGER IF NOT EXISTS medications_ai AFTER INSERT ON medications BEGIN
            INSERT INTO medications_fts(rowid, medication_id, patient_id, name, dosage, frequency, prescriber, notes)
            VALUES (new.id, new.id, new.patient_id, new.name, new.dosage, new.frequency, new.prescriber, new.notes);
        END;

        CREATE TRIGGER IF NOT EXISTS medications_ad AFTER DELETE ON medications BEGIN
            INSERT INTO medications_fts(medications_fts, rowid, medication_id, patient_id, name, dosage, frequency, prescriber, notes)
            VALUES ('delete', old.id, old.id, old.patient_id, old.name, old.dosage, old.frequency, old.prescriber, old.notes);
        END;

        CREATE TRIGGER IF NOT EXISTS medications_au AFTER UPDATE ON medications BEGIN
            INSERT INTO medications_fts(medications_fts, rowid, medication_id, patient_id, name, dosage, frequency, prescriber, notes)
            VALUES ('delete', old.id, old.id, old.patient_id, old.name, old.dosage, old.frequency, old.prescriber, old.notes);
            INSERT INTO medications_fts(rowid, medication_id, patient_id, name, dosage, frequency, prescriber, notes)
            VALUES (new.id, new.id, new.patient_id, new.name, new.dosage, new.frequency, new.prescriber, new.notes);
        END;

        CREATE TRIGGER IF NOT EXISTS labs_ai AFTER INSERT ON labs BEGIN
            INSERT INTO labs_fts(rowid, lab_id, patient_id, test_name, result, unit, notes)
            VALUES (new.id, new.id, new.patient_id, new.test_name, new.result, new.unit, new.notes);
        END;

        CREATE TRIGGER IF NOT EXISTS labs_ad AFTER DELETE ON labs BEGIN
            INSERT INTO labs_fts(labs_fts, rowid, lab_id, patient_id, test_name, result, unit, notes)
            VALUES ('delete', old.id, old.id, old.patient_id, old.test_name, old.result, old.unit, old.notes);
        END;

        CREATE TRIGGER IF NOT EXISTS labs_au AFTER UPDATE ON labs BEGIN
            INSERT INTO labs_fts(labs_fts, rowid, lab_id, patient_id, test_name, result, unit, notes)
            VALUES ('delete', old.id, old.id, old.patient_id, old.test_name, old.result, old.unit, old.notes);
            INSERT INTO labs_fts(rowid, lab_id, patient_id, test_name, result, unit, notes)
            VALUES (new.id, new.id, new.patient_id, new.test_name, new.result, new.unit, new.notes);
        END;
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

    // Add pharmacy/insurance columns to patients table if they don't exist
    let has_pharmacy: bool = conn
        .prepare("SELECT preferred_pharmacy FROM patients LIMIT 1")
        .is_ok();
    if !has_pharmacy {
        let _ = conn.execute("ALTER TABLE patients ADD COLUMN preferred_pharmacy TEXT", []);
        let _ = conn.execute("ALTER TABLE patients ADD COLUMN insurance_provider TEXT", []);
        let _ = conn.execute("ALTER TABLE patients ADD COLUMN insurance_policy_number TEXT", []);
        let _ = conn.execute("ALTER TABLE patients ADD COLUMN insurance_group_number TEXT", []);
    }

    // Add category column to diagnoses table if it doesn't exist
    let has_category: bool = conn
        .prepare("SELECT category FROM diagnoses LIMIT 1")
        .is_ok();
    if !has_category {
        let _ = conn.execute("ALTER TABLE diagnoses ADD COLUMN category TEXT", []);
    }

    // Add zen_mode_default column to user_settings table if it doesn't exist
    let has_zen_mode: bool = conn
        .prepare("SELECT zen_mode_default FROM user_settings LIMIT 1")
        .is_ok();
    if !has_zen_mode {
        let _ = conn.execute("ALTER TABLE user_settings ADD COLUMN zen_mode_default INTEGER DEFAULT 0", []);
    }

    // Rebuild FTS indexes for existing data
    rebuild_fts_indexes(&conn)?;

    Ok(conn)
}

// ============ FTS Search Functions ============

/// Rebuild FTS indexes from existing data
pub fn rebuild_fts_indexes(conn: &Connection) -> Result<()> {
    // Clear and rebuild patients_fts
    let _ = conn.execute("DELETE FROM patients_fts", []);
    let _ = conn.execute(
        "INSERT INTO patients_fts(rowid, patient_id, first_name, last_name, phone, email, address, ai_summary)
         SELECT id, id, first_name, last_name, phone, email, address, ai_summary FROM patients",
        []
    );

    // Clear and rebuild encounters_fts
    let _ = conn.execute("DELETE FROM encounters_fts", []);
    let _ = conn.execute(
        "INSERT INTO encounters_fts(rowid, encounter_id, patient_id, encounter_type, chief_complaint, summary, note_content, provider)
         SELECT id, id, patient_id, encounter_type, chief_complaint, summary, note_content, provider FROM encounters",
        []
    );

    // Clear and rebuild diagnoses_fts
    let _ = conn.execute("DELETE FROM diagnoses_fts", []);
    let _ = conn.execute(
        "INSERT INTO diagnoses_fts(rowid, diagnosis_id, patient_id, icd_code, description, category, notes)
         SELECT id, id, patient_id, icd_code, description, category, notes FROM diagnoses",
        []
    );

    // Clear and rebuild medications_fts
    let _ = conn.execute("DELETE FROM medications_fts", []);
    let _ = conn.execute(
        "INSERT INTO medications_fts(rowid, medication_id, patient_id, name, dosage, frequency, prescriber, notes)
         SELECT id, id, patient_id, name, dosage, frequency, prescriber, notes FROM medications",
        []
    );

    // Clear and rebuild labs_fts
    let _ = conn.execute("DELETE FROM labs_fts", []);
    let _ = conn.execute(
        "INSERT INTO labs_fts(rowid, lab_id, patient_id, test_name, result, unit, notes)
         SELECT id, id, patient_id, test_name, result, unit, notes FROM labs",
        []
    );

    Ok(())
}

/// Search result item with type information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub result_type: String,  // "patient", "encounter", "diagnosis", "medication", "lab"
    pub id: i64,
    pub patient_id: Option<i64>,
    pub title: String,
    pub subtitle: Option<String>,
    pub snippet: Option<String>,
    pub rank: f64,
}

/// Global search across all FTS tables
pub fn global_search(conn: &Connection, query: &str, limit: i64) -> Result<Vec<SearchResult>> {
    let mut results: Vec<SearchResult> = Vec::new();

    // Escape the query for FTS5 - wrap each word with quotes to handle special chars
    let escaped_query = query
        .split_whitespace()
        .map(|word| format!("\"{}\"*", word.replace("\"", "\"\"")))
        .collect::<Vec<_>>()
        .join(" ");

    if escaped_query.is_empty() {
        return Ok(results);
    }

    // Search patients
    let mut stmt = conn.prepare(
        "SELECT patient_id, first_name, last_name, phone, snippet(patients_fts, 4, '<mark>', '</mark>', '...', 32), rank
         FROM patients_fts WHERE patients_fts MATCH ?1
         ORDER BY rank LIMIT ?2"
    )?;

    let patient_results = stmt.query_map(params![&escaped_query, limit], |row| {
        Ok(SearchResult {
            result_type: "patient".to_string(),
            id: row.get(0)?,
            patient_id: Some(row.get(0)?),
            title: format!("{} {}", row.get::<_, String>(1)?, row.get::<_, String>(2)?),
            subtitle: row.get(3).ok(),
            snippet: row.get(4).ok(),
            rank: row.get(5)?,
        })
    })?;

    for result in patient_results {
        if let Ok(r) = result {
            results.push(r);
        }
    }

    // Search encounters
    let mut stmt = conn.prepare(
        "SELECT encounter_id, patient_id, encounter_type, chief_complaint, snippet(encounters_fts, 5, '<mark>', '</mark>', '...', 32), rank
         FROM encounters_fts WHERE encounters_fts MATCH ?1
         ORDER BY rank LIMIT ?2"
    )?;

    let encounter_results = stmt.query_map(params![&escaped_query, limit], |row| {
        Ok(SearchResult {
            result_type: "encounter".to_string(),
            id: row.get(0)?,
            patient_id: Some(row.get(1)?),
            title: row.get::<_, Option<String>>(3)?.unwrap_or_else(|| row.get::<_, String>(2).unwrap_or_default()),
            subtitle: Some(row.get::<_, String>(2)?),
            snippet: row.get(4).ok(),
            rank: row.get(5)?,
        })
    })?;

    for result in encounter_results {
        if let Ok(r) = result {
            results.push(r);
        }
    }

    // Search diagnoses
    let mut stmt = conn.prepare(
        "SELECT diagnosis_id, patient_id, icd_code, description, snippet(diagnoses_fts, 3, '<mark>', '</mark>', '...', 32), rank
         FROM diagnoses_fts WHERE diagnoses_fts MATCH ?1
         ORDER BY rank LIMIT ?2"
    )?;

    let diagnosis_results = stmt.query_map(params![&escaped_query, limit], |row| {
        Ok(SearchResult {
            result_type: "diagnosis".to_string(),
            id: row.get(0)?,
            patient_id: Some(row.get(1)?),
            title: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
            subtitle: row.get(2).ok(),
            snippet: row.get(4).ok(),
            rank: row.get(5)?,
        })
    })?;

    for result in diagnosis_results {
        if let Ok(r) = result {
            results.push(r);
        }
    }

    // Search medications
    let mut stmt = conn.prepare(
        "SELECT medication_id, patient_id, name, dosage, snippet(medications_fts, 2, '<mark>', '</mark>', '...', 32), rank
         FROM medications_fts WHERE medications_fts MATCH ?1
         ORDER BY rank LIMIT ?2"
    )?;

    let med_results = stmt.query_map(params![&escaped_query, limit], |row| {
        Ok(SearchResult {
            result_type: "medication".to_string(),
            id: row.get(0)?,
            patient_id: Some(row.get(1)?),
            title: row.get::<_, String>(2)?,
            subtitle: row.get(3).ok(),
            snippet: row.get(4).ok(),
            rank: row.get(5)?,
        })
    })?;

    for result in med_results {
        if let Ok(r) = result {
            results.push(r);
        }
    }

    // Search labs
    let mut stmt = conn.prepare(
        "SELECT lab_id, patient_id, test_name, result, snippet(labs_fts, 2, '<mark>', '</mark>', '...', 32), rank
         FROM labs_fts WHERE labs_fts MATCH ?1
         ORDER BY rank LIMIT ?2"
    )?;

    let lab_results = stmt.query_map(params![&escaped_query, limit], |row| {
        Ok(SearchResult {
            result_type: "lab".to_string(),
            id: row.get(0)?,
            patient_id: Some(row.get(1)?),
            title: row.get::<_, String>(2)?,
            subtitle: row.get(3).ok(),
            snippet: row.get(4).ok(),
            rank: row.get(5)?,
        })
    })?;

    for result in lab_results {
        if let Ok(r) = result {
            results.push(r);
        }
    }

    // Sort all results by rank (lower is better in FTS5)
    results.sort_by(|a, b| a.rank.partial_cmp(&b.rank).unwrap_or(std::cmp::Ordering::Equal));

    // Limit total results
    results.truncate(limit as usize);

    Ok(results)
}

/// Search within a specific patient's data
pub fn search_patient_data(conn: &Connection, patient_id: i64, query: &str, limit: i64) -> Result<Vec<SearchResult>> {
    let mut results: Vec<SearchResult> = Vec::new();

    let escaped_query = query
        .split_whitespace()
        .map(|word| format!("\"{}\"*", word.replace("\"", "\"\"")))
        .collect::<Vec<_>>()
        .join(" ");

    if escaped_query.is_empty() {
        return Ok(results);
    }

    // Search encounters for this patient
    let mut stmt = conn.prepare(
        "SELECT encounter_id, encounter_type, chief_complaint, snippet(encounters_fts, 5, '<mark>', '</mark>', '...', 32), rank
         FROM encounters_fts WHERE encounters_fts MATCH ?1 AND patient_id = ?2
         ORDER BY rank LIMIT ?3"
    )?;

    let encounter_results = stmt.query_map(params![&escaped_query, patient_id, limit], |row| {
        Ok(SearchResult {
            result_type: "encounter".to_string(),
            id: row.get(0)?,
            patient_id: Some(patient_id),
            title: row.get::<_, Option<String>>(2)?.unwrap_or_else(|| row.get::<_, String>(1).unwrap_or_default()),
            subtitle: Some(row.get::<_, String>(1)?),
            snippet: row.get(3).ok(),
            rank: row.get(4)?,
        })
    })?;

    for result in encounter_results {
        if let Ok(r) = result {
            results.push(r);
        }
    }

    // Search diagnoses for this patient
    let mut stmt = conn.prepare(
        "SELECT diagnosis_id, icd_code, description, snippet(diagnoses_fts, 3, '<mark>', '</mark>', '...', 32), rank
         FROM diagnoses_fts WHERE diagnoses_fts MATCH ?1 AND patient_id = ?2
         ORDER BY rank LIMIT ?3"
    )?;

    let diagnosis_results = stmt.query_map(params![&escaped_query, patient_id, limit], |row| {
        Ok(SearchResult {
            result_type: "diagnosis".to_string(),
            id: row.get(0)?,
            patient_id: Some(patient_id),
            title: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
            subtitle: row.get(1).ok(),
            snippet: row.get(3).ok(),
            rank: row.get(4)?,
        })
    })?;

    for result in diagnosis_results {
        if let Ok(r) = result {
            results.push(r);
        }
    }

    // Search medications for this patient
    let mut stmt = conn.prepare(
        "SELECT medication_id, name, dosage, snippet(medications_fts, 2, '<mark>', '</mark>', '...', 32), rank
         FROM medications_fts WHERE medications_fts MATCH ?1 AND patient_id = ?2
         ORDER BY rank LIMIT ?3"
    )?;

    let med_results = stmt.query_map(params![&escaped_query, patient_id, limit], |row| {
        Ok(SearchResult {
            result_type: "medication".to_string(),
            id: row.get(0)?,
            patient_id: Some(patient_id),
            title: row.get::<_, String>(1)?,
            subtitle: row.get(2).ok(),
            snippet: row.get(3).ok(),
            rank: row.get(4)?,
        })
    })?;

    for result in med_results {
        if let Ok(r) = result {
            results.push(r);
        }
    }

    // Search labs for this patient
    let mut stmt = conn.prepare(
        "SELECT lab_id, test_name, result, snippet(labs_fts, 2, '<mark>', '</mark>', '...', 32), rank
         FROM labs_fts WHERE labs_fts MATCH ?1 AND patient_id = ?2
         ORDER BY rank LIMIT ?3"
    )?;

    let lab_results = stmt.query_map(params![&escaped_query, patient_id, limit], |row| {
        Ok(SearchResult {
            result_type: "lab".to_string(),
            id: row.get(0)?,
            patient_id: Some(patient_id),
            title: row.get::<_, String>(1)?,
            subtitle: row.get(2).ok(),
            snippet: row.get(3).ok(),
            rank: row.get(4)?,
        })
    })?;

    for result in lab_results {
        if let Ok(r) = result {
            results.push(r);
        }
    }

    // Sort by rank
    results.sort_by(|a, b| a.rank.partial_cmp(&b.rank).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(limit as usize);

    Ok(results)
}

/// Quick patient search (for patient list filtering)
pub fn quick_search_patients(conn: &Connection, query: &str, limit: i64) -> Result<Vec<Patient>> {
    if query.trim().is_empty() {
        return get_all_patients(conn);
    }

    let escaped_query = query
        .split_whitespace()
        .map(|word| format!("\"{}\"*", word.replace("\"", "\"\"")))
        .collect::<Vec<_>>()
        .join(" ");

    let mut stmt = conn.prepare(
        "SELECT p.id, p.first_name, p.last_name, p.dob, p.sex, p.gender, p.address, p.phone, p.email,
                p.photo_url, p.ai_summary, p.preferred_pharmacy, p.insurance_provider,
                p.insurance_policy_number, p.insurance_group_number
         FROM patients p
         INNER JOIN patients_fts fts ON p.id = fts.patient_id
         WHERE patients_fts MATCH ?1
         ORDER BY fts.rank
         LIMIT ?2"
    )?;

    let patients = stmt.query_map(params![&escaped_query, limit], |row| {
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
            preferred_pharmacy: row.get(11)?,
            insurance_provider: row.get(12)?,
            insurance_policy_number: row.get(13)?,
            insurance_group_number: row.get(14)?,
        })
    })?;

    patients.collect()
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
    pub preferred_pharmacy: Option<String>,
    pub insurance_provider: Option<String>,
    pub insurance_policy_number: Option<String>,
    pub insurance_group_number: Option<String>,
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
    pub category: Option<String>,
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

// ============ Prescription Struct ============

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prescription {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub medication_id: i64,
    pub quantity: i32,
    pub days_supply: i32,
    pub refills: i32,
    pub sig: String,
    pub pharmacy: Option<String>,
    pub prescriber_id: Option<i64>,
    pub status: Option<String>,
    pub prescribed_date: Option<String>,
    pub filled_date: Option<String>,
    pub notes: Option<String>,
}

// ============ User/Provider Structs ============

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub degree_type: Option<String>,
    pub specialty: Option<String>,
    pub subspecialty: Option<String>,
    pub npi_number: Option<String>,
    pub photo_url: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEducation {
    pub id: Option<i64>,
    pub user_id: i64,
    pub education_type: String,
    pub institution: String,
    pub degree: Option<String>,
    pub field_of_study: Option<String>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBadge {
    pub id: Option<i64>,
    pub user_id: i64,
    pub badge_name: String,
    pub badge_type: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub awarded_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub id: Option<i64>,
    pub user_id: i64,
    pub language: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub email_notifications: Option<bool>,
    pub sms_notifications: Option<bool>,
    pub two_factor_enabled: Option<bool>,
    pub zen_mode_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFullData {
    pub user: User,
    pub education: Vec<UserEducation>,
    pub badges: Vec<UserBadge>,
    pub settings: UserSettings,
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
        "INSERT INTO patients (first_name, last_name, dob, sex, gender, address, phone, email, preferred_pharmacy, insurance_provider, insurance_policy_number, insurance_group_number)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            patient.first_name,
            patient.last_name,
            patient.dob,
            patient.sex,
            patient.gender,
            patient.address,
            patient.phone,
            patient.email,
            patient.preferred_pharmacy,
            patient.insurance_provider,
            patient.insurance_policy_number,
            patient.insurance_group_number,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_all_patients(conn: &Connection) -> Result<Vec<Patient>> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, last_name, dob, sex, gender, address, phone, email, photo_url, ai_summary, preferred_pharmacy, insurance_provider, insurance_policy_number, insurance_group_number FROM patients"
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
            preferred_pharmacy: row.get(11)?,
            insurance_provider: row.get(12)?,
            insurance_policy_number: row.get(13)?,
            insurance_group_number: row.get(14)?,
        })
    })?;

    patients.collect()
}

pub fn get_patient_by_id(conn: &Connection, id: i64) -> Result<Option<Patient>> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, last_name, dob, sex, gender, address, phone, email, photo_url, ai_summary, preferred_pharmacy, insurance_provider, insurance_policy_number, insurance_group_number
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
            preferred_pharmacy: row.get(11)?,
            insurance_provider: row.get(12)?,
            insurance_policy_number: row.get(13)?,
            insurance_group_number: row.get(14)?,
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
        "SELECT id, patient_id, name, icd_code, onset_date, status, category
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
            category: row.get(6)?,
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

pub fn create_encounter(conn: &Connection, encounter: &Encounter) -> Result<i64> {
    conn.execute(
        "INSERT INTO encounters (patient_id, encounter_date, encounter_type, chief_complaint, summary, note_content, provider, location)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            encounter.patient_id,
            encounter.encounter_date,
            encounter.encounter_type,
            encounter.chief_complaint,
            encounter.summary,
            encounter.note_content,
            encounter.provider,
            encounter.location,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_encounter(conn: &Connection, encounter: &Encounter) -> Result<()> {
    conn.execute(
        "UPDATE encounters SET
            encounter_date = ?1,
            encounter_type = ?2,
            chief_complaint = ?3,
            summary = ?4,
            note_content = ?5,
            provider = ?6,
            location = ?7
         WHERE id = ?8",
        params![
            encounter.encounter_date,
            encounter.encounter_type,
            encounter.chief_complaint,
            encounter.summary,
            encounter.note_content,
            encounter.provider,
            encounter.location,
            encounter.id,
        ],
    )?;
    Ok(())
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

pub fn create_allergy(conn: &Connection, allergy: &Allergy) -> Result<i64> {
    conn.execute(
        "INSERT INTO allergies (patient_id, allergen, reaction, severity, status)
         VALUES (?1, ?2, ?3, ?4, 'active')",
        params![allergy.patient_id, allergy.allergen, allergy.reaction, allergy.severity],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_allergy(conn: &Connection, allergy: &Allergy) -> Result<()> {
    conn.execute(
        "UPDATE allergies SET allergen = ?1, reaction = ?2, severity = ?3 WHERE id = ?4",
        params![allergy.allergen, allergy.reaction, allergy.severity, allergy.id],
    )?;
    Ok(())
}

pub fn delete_allergy(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM allergies WHERE id = ?1", params![id])?;
    Ok(())
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

pub fn create_vaccination(conn: &Connection, vax: &Vaccination) -> Result<i64> {
    conn.execute(
        "INSERT INTO vaccinations (patient_id, vaccine_name, date_given)
         VALUES (?1, ?2, ?3)",
        params![vax.patient_id, vax.vaccine_name, vax.date_given],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_vaccination(conn: &Connection, vax: &Vaccination) -> Result<()> {
    conn.execute(
        "UPDATE vaccinations SET vaccine_name = ?1, date_given = ?2 WHERE id = ?3",
        params![vax.vaccine_name, vax.date_given, vax.id],
    )?;
    Ok(())
}

pub fn delete_vaccination(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM vaccinations WHERE id = ?1", params![id])?;
    Ok(())
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

pub fn create_social_history(conn: &Connection, history: &SocialHistory) -> Result<i64> {
    conn.execute(
        "INSERT INTO social_history (patient_id, category, detail, status)
         VALUES (?1, ?2, ?3, ?4)",
        params![history.patient_id, history.category, history.detail, history.status],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_social_history(conn: &Connection, history: &SocialHistory) -> Result<()> {
    conn.execute(
        "UPDATE social_history SET category = ?1, detail = ?2, status = ?3 WHERE id = ?4",
        params![history.category, history.detail, history.status, history.id],
    )?;
    Ok(())
}

pub fn delete_social_history(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM social_history WHERE id = ?1", params![id])?;
    Ok(())
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

pub fn create_family_history(conn: &Connection, history: &FamilyHistory) -> Result<i64> {
    conn.execute(
        "INSERT INTO family_history (patient_id, relation, condition, age_at_onset)
         VALUES (?1, ?2, ?3, ?4)",
        params![history.patient_id, history.relation, history.condition, history.age_at_onset],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_family_history(conn: &Connection, history: &FamilyHistory) -> Result<()> {
    conn.execute(
        "UPDATE family_history SET relation = ?1, condition = ?2, age_at_onset = ?3 WHERE id = ?4",
        params![history.relation, history.condition, history.age_at_onset, history.id],
    )?;
    Ok(())
}

pub fn delete_family_history(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM family_history WHERE id = ?1", params![id])?;
    Ok(())
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

// ============ Clear Patient Detail Data ============

pub fn clear_patient_detail_data(conn: &Connection, patient_id: i64) -> Result<()> {
    // Delete all related data for this patient to allow reseeding
    conn.execute("DELETE FROM diagnosis_medications WHERE diagnosis_id IN (SELECT id FROM diagnoses WHERE patient_id = ?1)", params![patient_id])?;
    conn.execute("DELETE FROM diagnoses WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM medications WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM vitals WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM labs WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM clinical_scores WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM encounters WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM allergies WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM vaccinations WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM social_history WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM family_history WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM todos WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM goals WHERE patient_id = ?1", params![patient_id])?;
    conn.execute("DELETE FROM timeline_events WHERE patient_id = ?1", params![patient_id])?;
    Ok(())
}

// ============ User CRUD Operations ============

pub fn create_user(conn: &Connection, user: &User) -> Result<i64> {
    conn.execute(
        "INSERT INTO users (username, password_hash, first_name, last_name, degree_type, specialty, subspecialty, npi_number, photo_url, bio)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            user.username,
            user.password_hash,
            user.first_name,
            user.last_name,
            user.degree_type,
            user.specialty,
            user.subspecialty,
            user.npi_number,
            user.photo_url,
            user.bio,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_user_by_id(conn: &Connection, id: i64) -> Result<Option<User>> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, first_name, last_name, degree_type, specialty, subspecialty, npi_number, photo_url, bio
         FROM users WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(User {
            id: Some(row.get(0)?),
            username: row.get(1)?,
            password_hash: row.get(2)?,
            first_name: row.get(3)?,
            last_name: row.get(4)?,
            degree_type: row.get(5)?,
            specialty: row.get(6)?,
            subspecialty: row.get(7)?,
            npi_number: row.get(8)?,
            photo_url: row.get(9)?,
            bio: row.get(10)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_current_user(conn: &Connection) -> Result<Option<User>> {
    // For now, get the first user (single-user app)
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, first_name, last_name, degree_type, specialty, subspecialty, npi_number, photo_url, bio
         FROM users ORDER BY id LIMIT 1"
    )?;

    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        Ok(Some(User {
            id: Some(row.get(0)?),
            username: row.get(1)?,
            password_hash: row.get(2)?,
            first_name: row.get(3)?,
            last_name: row.get(4)?,
            degree_type: row.get(5)?,
            specialty: row.get(6)?,
            subspecialty: row.get(7)?,
            npi_number: row.get(8)?,
            photo_url: row.get(9)?,
            bio: row.get(10)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn update_user(conn: &Connection, user: &User) -> Result<()> {
    conn.execute(
        "UPDATE users SET
            username = ?1,
            first_name = ?2,
            last_name = ?3,
            degree_type = ?4,
            specialty = ?5,
            subspecialty = ?6,
            npi_number = ?7,
            photo_url = ?8,
            bio = ?9,
            updated_at = datetime('now', 'localtime')
         WHERE id = ?10",
        params![
            user.username,
            user.first_name,
            user.last_name,
            user.degree_type,
            user.specialty,
            user.subspecialty,
            user.npi_number,
            user.photo_url,
            user.bio,
            user.id,
        ],
    )?;
    Ok(())
}

pub fn update_user_password(conn: &Connection, user_id: i64, new_password_hash: &str) -> Result<()> {
    conn.execute(
        "UPDATE users SET password_hash = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2",
        params![new_password_hash, user_id],
    )?;
    Ok(())
}

// ============ User Education CRUD Operations ============

pub fn get_education_for_user(conn: &Connection, user_id: i64) -> Result<Vec<UserEducation>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, education_type, institution, degree, field_of_study, start_year, end_year
         FROM user_education WHERE user_id = ?1
         ORDER BY end_year DESC"
    )?;

    let education = stmt.query_map(params![user_id], |row| {
        Ok(UserEducation {
            id: Some(row.get(0)?),
            user_id: row.get(1)?,
            education_type: row.get(2)?,
            institution: row.get(3)?,
            degree: row.get(4)?,
            field_of_study: row.get(5)?,
            start_year: row.get(6)?,
            end_year: row.get(7)?,
        })
    })?;

    education.collect()
}

pub fn create_user_education(conn: &Connection, edu: &UserEducation) -> Result<i64> {
    conn.execute(
        "INSERT INTO user_education (user_id, education_type, institution, degree, field_of_study, start_year, end_year)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            edu.user_id,
            edu.education_type,
            edu.institution,
            edu.degree,
            edu.field_of_study,
            edu.start_year,
            edu.end_year,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

// ============ User Badges CRUD Operations ============

pub fn get_badges_for_user(conn: &Connection, user_id: i64) -> Result<Vec<UserBadge>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, badge_name, badge_type, description, icon, color, awarded_date
         FROM user_badges WHERE user_id = ?1
         ORDER BY awarded_date DESC"
    )?;

    let badges = stmt.query_map(params![user_id], |row| {
        Ok(UserBadge {
            id: Some(row.get(0)?),
            user_id: row.get(1)?,
            badge_name: row.get(2)?,
            badge_type: row.get(3)?,
            description: row.get(4)?,
            icon: row.get(5)?,
            color: row.get(6)?,
            awarded_date: row.get(7)?,
        })
    })?;

    badges.collect()
}

pub fn create_user_badge(conn: &Connection, badge: &UserBadge) -> Result<i64> {
    conn.execute(
        "INSERT INTO user_badges (user_id, badge_name, badge_type, description, icon, color, awarded_date)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            badge.user_id,
            badge.badge_name,
            badge.badge_type,
            badge.description,
            badge.icon,
            badge.color,
            badge.awarded_date,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

// ============ User Settings CRUD Operations ============

pub fn get_settings_for_user(conn: &Connection, user_id: i64) -> Result<Option<UserSettings>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, language, notifications_enabled, email_notifications, sms_notifications, two_factor_enabled, zen_mode_default
         FROM user_settings WHERE user_id = ?1"
    )?;

    let mut rows = stmt.query(params![user_id])?;

    if let Some(row) = rows.next()? {
        let notif_int: Option<i32> = row.get(3)?;
        let email_int: Option<i32> = row.get(4)?;
        let sms_int: Option<i32> = row.get(5)?;
        let tfa_int: Option<i32> = row.get(6)?;
        let zen_int: Option<i32> = row.get(7)?;

        Ok(Some(UserSettings {
            id: Some(row.get(0)?),
            user_id: row.get(1)?,
            language: row.get(2)?,
            notifications_enabled: notif_int.map(|v| v != 0),
            email_notifications: email_int.map(|v| v != 0),
            sms_notifications: sms_int.map(|v| v != 0),
            two_factor_enabled: tfa_int.map(|v| v != 0),
            zen_mode_default: zen_int.map(|v| v != 0),
        }))
    } else {
        // Create default settings if none exist
        conn.execute(
            "INSERT INTO user_settings (user_id) VALUES (?1)",
            params![user_id],
        )?;
        Ok(Some(UserSettings {
            id: Some(conn.last_insert_rowid()),
            user_id,
            language: Some("en".to_string()),
            notifications_enabled: Some(true),
            email_notifications: Some(true),
            sms_notifications: Some(false),
            two_factor_enabled: Some(false),
            zen_mode_default: Some(false),
        }))
    }
}

pub fn update_user_settings(conn: &Connection, settings: &UserSettings) -> Result<()> {
    conn.execute(
        "UPDATE user_settings SET
            language = ?1,
            notifications_enabled = ?2,
            email_notifications = ?3,
            sms_notifications = ?4,
            two_factor_enabled = ?5,
            zen_mode_default = ?6,
            updated_at = datetime('now', 'localtime')
         WHERE user_id = ?7",
        params![
            settings.language,
            settings.notifications_enabled.map(|b| if b { 1 } else { 0 }),
            settings.email_notifications.map(|b| if b { 1 } else { 0 }),
            settings.sms_notifications.map(|b| if b { 1 } else { 0 }),
            settings.two_factor_enabled.map(|b| if b { 1 } else { 0 }),
            settings.zen_mode_default.map(|b| if b { 1 } else { 0 }),
            settings.user_id,
        ],
    )?;
    Ok(())
}

// ============ User Full Data (Aggregated) ============

pub fn get_user_full_data(conn: &Connection, user_id: i64) -> Result<Option<UserFullData>> {
    let user = match get_user_by_id(conn, user_id)? {
        Some(u) => u,
        None => return Ok(None),
    };

    let education = get_education_for_user(conn, user_id)?;
    let badges = get_badges_for_user(conn, user_id)?;
    let settings = get_settings_for_user(conn, user_id)?.unwrap_or(UserSettings {
        id: None,
        user_id,
        language: Some("en".to_string()),
        notifications_enabled: Some(true),
        email_notifications: Some(true),
        sms_notifications: Some(false),
        two_factor_enabled: Some(false),
        zen_mode_default: Some(false),
    });

    Ok(Some(UserFullData {
        user,
        education,
        badges,
        settings,
    }))
}

pub fn get_current_user_full_data(conn: &Connection) -> Result<Option<UserFullData>> {
    let user = match get_current_user(conn)? {
        Some(u) => u,
        None => return Ok(None),
    };

    let user_id = user.id.unwrap_or(0);
    let education = get_education_for_user(conn, user_id)?;
    let badges = get_badges_for_user(conn, user_id)?;
    let settings = get_settings_for_user(conn, user_id)?.unwrap_or(UserSettings {
        id: None,
        user_id,
        language: Some("en".to_string()),
        notifications_enabled: Some(true),
        email_notifications: Some(true),
        sms_notifications: Some(false),
        two_factor_enabled: Some(false),
        zen_mode_default: Some(false),
    });

    Ok(Some(UserFullData {
        user,
        education,
        badges,
        settings,
    }))
}

// ============ Seed User Data ============

pub fn seed_user_data(conn: &Connection) -> Result<()> {
    // Check if a user already exists
    let existing = get_current_user(conn)?;
    if existing.is_some() {
        return Ok(());
    }

    // Create Dr. Madeline Chu
    let user = User {
        id: None,
        username: "mchu".to_string(),
        password_hash: "hashed_password_placeholder".to_string(),
        first_name: "Madeline".to_string(),
        last_name: "Chu".to_string(),
        degree_type: Some("MD".to_string()),
        specialty: Some("Psychiatry".to_string()),
        subspecialty: Some("Child and Adolescent Psychiatry".to_string()),
        npi_number: Some("1234567890".to_string()),
        photo_url: None,
        bio: Some("Dr. Madeline Chu is a board-certified child and adolescent psychiatrist with a passion for helping young patients and their families navigate mental health challenges. She believes in a collaborative, evidence-based approach that incorporates both therapeutic interventions and, when appropriate, medication management. Outside of clinical practice, she enjoys hiking, watercolor painting, and volunteering at local community mental health organizations.".to_string()),
    };

    let user_id = create_user(conn, &user)?;

    // Create default settings
    get_settings_for_user(conn, user_id)?;

    // Add education history
    let education_entries = vec![
        UserEducation {
            id: None,
            user_id,
            education_type: "Medical School".to_string(),
            institution: "Stanford University School of Medicine".to_string(),
            degree: Some("MD".to_string()),
            field_of_study: Some("Medicine".to_string()),
            start_year: Some(2010),
            end_year: Some(2014),
        },
        UserEducation {
            id: None,
            user_id,
            education_type: "Residency".to_string(),
            institution: "Massachusetts General Hospital".to_string(),
            degree: None,
            field_of_study: Some("Psychiatry".to_string()),
            start_year: Some(2014),
            end_year: Some(2018),
        },
        UserEducation {
            id: None,
            user_id,
            education_type: "Fellowship".to_string(),
            institution: "Boston Children's Hospital".to_string(),
            degree: None,
            field_of_study: Some("Child and Adolescent Psychiatry".to_string()),
            start_year: Some(2018),
            end_year: Some(2020),
        },
        UserEducation {
            id: None,
            user_id,
            education_type: "Undergraduate".to_string(),
            institution: "University of California, Berkeley".to_string(),
            degree: Some("BA".to_string()),
            field_of_study: Some("Psychology".to_string()),
            start_year: Some(2006),
            end_year: Some(2010),
        },
    ];

    for edu in &education_entries {
        create_user_education(conn, edu)?;
    }

    // Add badges/awards
    let badges = vec![
        UserBadge {
            id: None,
            user_id,
            badge_name: "Board Certified".to_string(),
            badge_type: "certification".to_string(),
            description: Some("American Board of Psychiatry and Neurology".to_string()),
            icon: Some("shield-check".to_string()),
            color: Some("#3b82f6".to_string()),
            awarded_date: Some("2018-08-15".to_string()),
        },
        UserBadge {
            id: None,
            user_id,
            badge_name: "CAP Certified".to_string(),
            badge_type: "certification".to_string(),
            description: Some("Child & Adolescent Psychiatry Subspecialty".to_string()),
            icon: Some("academic-cap".to_string()),
            color: Some("#8b5cf6".to_string()),
            awarded_date: Some("2020-06-20".to_string()),
        },
        UserBadge {
            id: None,
            user_id,
            badge_name: "Excellence in Teaching".to_string(),
            badge_type: "award".to_string(),
            description: Some("MGH Psychiatry Residency Program".to_string()),
            icon: Some("star".to_string()),
            color: Some("#f59e0b".to_string()),
            awarded_date: Some("2017-05-15".to_string()),
        },
        UserBadge {
            id: None,
            user_id,
            badge_name: "Research Fellow".to_string(),
            badge_type: "honor".to_string(),
            description: Some("NIMH Early Career Research Award".to_string()),
            icon: Some("beaker".to_string()),
            color: Some("#10b981".to_string()),
            awarded_date: Some("2021-03-01".to_string()),
        },
        UserBadge {
            id: None,
            user_id,
            badge_name: "Patient Choice Award".to_string(),
            badge_type: "award".to_string(),
            description: Some("Vitals.com - 3 consecutive years".to_string()),
            icon: Some("heart".to_string()),
            color: Some("#ec4899".to_string()),
            awarded_date: Some("2023-01-10".to_string()),
        },
    ];

    for badge in &badges {
        create_user_badge(conn, badge)?;
    }

    Ok(())
}

// ============ Seed Patient Detail Test Data ============

pub fn seed_patient_detail_test_data(conn: &Connection, patient_id: i64, force_reseed: bool) -> Result<()> {
    // Check if data already exists for this patient
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM diagnoses WHERE patient_id = ?1",
        params![patient_id],
        |row| row.get(0),
    )?;

    if count > 0 {
        if force_reseed {
            // Clear existing data first
            clear_patient_detail_data(conn, patient_id)?;
        } else {
            return Ok(()); // Already seeded
        }
    }

    // Update patient with AI summary and pharmacy/insurance info
    conn.execute(
        "UPDATE patients SET ai_summary = ?1, preferred_pharmacy = ?2, insurance_provider = ?3, insurance_policy_number = ?4, insurance_group_number = ?5 WHERE id = ?6",
        params![
            "Middle-aged patient managing hypertension, heart failure, and depression. Responds well to beta-blocker therapy with improving cardiac function and PHQ-9 scores.",
            "CVS Pharmacy - 1234 Main St, Springfield",
            "Blue Cross Blue Shield",
            "XYZ123456789",
            "GRP001234",
            patient_id
        ],
    )?;

    // Seed diagnoses with categories
    // Categories: cardiac (red), pulm (blue), gi (brown), neuro (orange), psych (purple),
    //             renal (yellow), endocrine (orange), obgyn (pink), oncology (lime),
    //             heme (maroon), msk (gray), immune (green), social (beige)
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, onset_date, status, category)
         VALUES (?1, 'Hypertension', 'I10', '2020-03-15', 'active', 'cardiac')",
        params![patient_id],
    )?;
    let htn_id = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, onset_date, status, category)
         VALUES (?1, 'Heart Failure with Reduced EF', 'I50.22', '2023-06-10', 'active', 'cardiac')",
        params![patient_id],
    )?;
    let hfref_id = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, onset_date, status, category)
         VALUES (?1, 'Major Depressive Disorder', 'F33.0', '2019-08-20', 'active', 'psych')",
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

// ============================================================================
// Patient Lists
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatientList {
    pub id: Option<i64>,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_default: bool,
    pub sort_order: i64,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatientListMember {
    pub id: Option<i64>,
    pub list_id: i64,
    pub patient_id: i64,
    pub added_at: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatientListColumn {
    pub id: Option<i64>,
    pub list_id: i64,
    pub column_key: String,
    pub column_label: String,
    pub column_type: Option<String>,
    pub is_visible: bool,
    pub sort_order: i64,
    pub width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientListWithPatients {
    pub list: PatientList,
    pub columns: Vec<PatientListColumn>,
    pub patients: Vec<Patient>,
}

// Default columns for patient lists
pub fn get_default_columns() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("name", "Name", "text"),
        ("dob", "DOB", "date"),
        ("sex", "Sex", "text"),
        ("phone", "Phone", "text"),
        ("last_visit", "Last Visit", "date"),
        ("next_appointment", "Next Appt", "date"),
        ("primary_diagnosis", "Primary Dx", "text"),
    ]
}

pub fn create_patient_list(conn: &Connection, list: &PatientList) -> Result<i64> {
    conn.execute(
        "INSERT INTO patient_lists (user_id, name, description, color, icon, is_default, sort_order)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            list.user_id,
            list.name,
            list.description,
            list.color,
            list.icon,
            list.is_default,
            list.sort_order,
        ],
    )?;
    let list_id = conn.last_insert_rowid();

    // Add default columns
    for (i, (key, label, col_type)) in get_default_columns().iter().enumerate() {
        conn.execute(
            "INSERT INTO patient_list_columns (list_id, column_key, column_label, column_type, is_visible, sort_order)
             VALUES (?1, ?2, ?3, ?4, 1, ?5)",
            params![list_id, key, label, col_type, i as i64],
        )?;
    }

    Ok(list_id)
}

pub fn get_patient_lists_for_user(conn: &Connection, user_id: i64) -> Result<Vec<PatientList>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, name, description, color, icon, is_default, sort_order
         FROM patient_lists WHERE user_id = ?1 ORDER BY sort_order, name"
    )?;

    let lists = stmt.query_map(params![user_id], |row| {
        Ok(PatientList {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            color: row.get(4)?,
            icon: row.get(5)?,
            is_default: row.get(6)?,
            sort_order: row.get(7)?,
        })
    })?.collect::<Result<Vec<_>>>()?;

    Ok(lists)
}

pub fn get_patient_list_by_id(conn: &Connection, list_id: i64) -> Result<Option<PatientList>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, name, description, color, icon, is_default, sort_order
         FROM patient_lists WHERE id = ?1"
    )?;

    let mut rows = stmt.query(params![list_id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(PatientList {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            color: row.get(4)?,
            icon: row.get(5)?,
            is_default: row.get(6)?,
            sort_order: row.get(7)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn update_patient_list(conn: &Connection, list: &PatientList) -> Result<()> {
    conn.execute(
        "UPDATE patient_lists SET name = ?1, description = ?2, color = ?3, icon = ?4, sort_order = ?5, updated_at = datetime('now', 'localtime')
         WHERE id = ?6",
        params![list.name, list.description, list.color, list.icon, list.sort_order, list.id],
    )?;
    Ok(())
}

pub fn delete_patient_list(conn: &Connection, list_id: i64) -> Result<()> {
    conn.execute("DELETE FROM patient_lists WHERE id = ?1", params![list_id])?;
    Ok(())
}

pub fn get_columns_for_list(conn: &Connection, list_id: i64) -> Result<Vec<PatientListColumn>> {
    let mut stmt = conn.prepare(
        "SELECT id, list_id, column_key, column_label, column_type, is_visible, sort_order, width
         FROM patient_list_columns WHERE list_id = ?1 ORDER BY sort_order"
    )?;

    let columns = stmt.query_map(params![list_id], |row| {
        Ok(PatientListColumn {
            id: row.get(0)?,
            list_id: row.get(1)?,
            column_key: row.get(2)?,
            column_label: row.get(3)?,
            column_type: row.get(4)?,
            is_visible: row.get(5)?,
            sort_order: row.get(6)?,
            width: row.get(7)?,
        })
    })?.collect::<Result<Vec<_>>>()?;

    Ok(columns)
}

pub fn update_list_columns(conn: &Connection, list_id: i64, columns: &[PatientListColumn]) -> Result<()> {
    // Delete existing columns
    conn.execute("DELETE FROM patient_list_columns WHERE list_id = ?1", params![list_id])?;

    // Insert new columns
    for col in columns {
        conn.execute(
            "INSERT INTO patient_list_columns (list_id, column_key, column_label, column_type, is_visible, sort_order, width)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![list_id, col.column_key, col.column_label, col.column_type, col.is_visible, col.sort_order, col.width],
        )?;
    }

    Ok(())
}

pub fn add_patient_to_list(conn: &Connection, list_id: i64, patient_id: i64, notes: Option<&str>) -> Result<i64> {
    conn.execute(
        "INSERT OR IGNORE INTO patient_list_members (list_id, patient_id, notes)
         VALUES (?1, ?2, ?3)",
        params![list_id, patient_id, notes],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn remove_patient_from_list(conn: &Connection, list_id: i64, patient_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM patient_list_members WHERE list_id = ?1 AND patient_id = ?2",
        params![list_id, patient_id],
    )?;
    Ok(())
}

pub fn get_patients_in_list(conn: &Connection, list_id: i64) -> Result<Vec<Patient>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.first_name, p.last_name, p.dob, p.sex, p.gender, p.address, p.phone, p.email, p.photo_url, p.ai_summary,
                p.preferred_pharmacy, p.insurance_provider, p.insurance_policy_number, p.insurance_group_number
         FROM patients p
         INNER JOIN patient_list_members plm ON p.id = plm.patient_id
         WHERE plm.list_id = ?1
         ORDER BY p.last_name, p.first_name"
    )?;

    let patients = stmt.query_map(params![list_id], |row| {
        Ok(Patient {
            id: row.get(0)?,
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
            preferred_pharmacy: row.get(11)?,
            insurance_provider: row.get(12)?,
            insurance_policy_number: row.get(13)?,
            insurance_group_number: row.get(14)?,
        })
    })?.collect::<Result<Vec<_>>>()?;

    Ok(patients)
}

pub fn get_patient_list_with_patients(conn: &Connection, list_id: i64) -> Result<Option<PatientListWithPatients>> {
    let list = get_patient_list_by_id(conn, list_id)?;
    if let Some(list) = list {
        let columns = get_columns_for_list(conn, list_id)?;
        let patients = get_patients_in_list(conn, list_id)?;
        Ok(Some(PatientListWithPatients { list, columns, patients }))
    } else {
        Ok(None)
    }
}

// ============ Prescription Functions ============

/// Create a new prescription
pub fn create_prescription(conn: &Connection, prescription: &Prescription) -> Result<i64> {
    conn.execute(
        "INSERT INTO prescriptions (patient_id, medication_id, quantity, days_supply, refills, sig, pharmacy, prescriber_id, status, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            prescription.patient_id,
            prescription.medication_id,
            prescription.quantity,
            prescription.days_supply,
            prescription.refills,
            prescription.sig,
            prescription.pharmacy,
            prescription.prescriber_id,
            prescription.status.as_deref().unwrap_or("sent"),
            prescription.notes,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Create multiple prescriptions at once (batch)
pub fn create_prescriptions_batch(conn: &Connection, prescriptions: &[Prescription]) -> Result<Vec<i64>> {
    let mut ids = Vec::new();
    for rx in prescriptions {
        let id = create_prescription(conn, rx)?;
        ids.push(id);
    }
    Ok(ids)
}

/// Get prescriptions for a patient
pub fn get_prescriptions_for_patient(conn: &Connection, patient_id: i64) -> Result<Vec<Prescription>> {
    let mut stmt = conn.prepare(
        "SELECT id, patient_id, medication_id, quantity, days_supply, refills, sig, pharmacy,
                prescriber_id, status, prescribed_date, filled_date, notes
         FROM prescriptions WHERE patient_id = ?1 ORDER BY prescribed_date DESC"
    )?;

    let prescriptions = stmt.query_map(params![patient_id], |row| {
        Ok(Prescription {
            id: row.get(0)?,
            patient_id: row.get(1)?,
            medication_id: row.get(2)?,
            quantity: row.get(3)?,
            days_supply: row.get(4)?,
            refills: row.get(5)?,
            sig: row.get(6)?,
            pharmacy: row.get(7)?,
            prescriber_id: row.get(8)?,
            status: row.get(9)?,
            prescribed_date: row.get(10)?,
            filled_date: row.get(11)?,
            notes: row.get(12)?,
        })
    })?.collect::<Result<Vec<_>>>()?;

    Ok(prescriptions)
}

pub fn seed_patient_lists(conn: &Connection, user_id: i64) -> Result<()> {
    // Check if lists already exist for this user
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM patient_lists WHERE user_id = ?1",
        params![user_id],
        |row| row.get(0),
    )?;

    if count > 0 {
        return Ok(());
    }

    // Create diverse patients first
    let patients_data = vec![
        // Resident Clinic patients (general outpatient)
        ("Marcus", "Williams", "1978-05-12", "M", "(555) 234-5678", "marcus.williams@email.com", "456 Oak Street, Springfield, IL 62701"),
        ("Jennifer", "Garcia", "1985-09-23", "F", "(555) 345-6789", "jgarcia@email.com", "789 Maple Ave, Springfield, IL 62702"),
        ("Robert", "Chen", "1952-01-30", "M", "(555) 456-7890", "rchen1952@email.com", "321 Pine Road, Springfield, IL 62703"),
        ("Aisha", "Johnson", "1990-11-15", "F", "(555) 567-8901", "aisha.j@email.com", "654 Elm Court, Springfield, IL 62704"),

        // Pain Clinic patients
        ("Thomas", "Anderson", "1965-03-08", "M", "(555) 678-9012", "tanderson@email.com", "987 Cedar Lane, Springfield, IL 62705"),
        ("Maria", "Rodriguez", "1973-07-19", "F", "(555) 789-0123", "mrodriguez@email.com", "147 Birch Drive, Springfield, IL 62706"),
        ("David", "Kim", "1958-12-04", "M", "(555) 890-1234", "dkim58@email.com", "258 Willow Way, Springfield, IL 62707"),

        // Yellow Team (hospital/inpatient)
        ("Patricia", "Brown", "1945-06-22", "F", "(555) 901-2345", "pbrown45@email.com", "369 Ash Street, Springfield, IL 62708"),
        ("James", "Wilson", "1938-02-14", "M", "(555) 012-3456", "jwilson38@email.com", "741 Spruce Ave, Springfield, IL 62709"),
        ("Linda", "Martinez", "1962-08-31", "F", "(555) 123-4567", "lmartinez@email.com", "852 Walnut Road, Springfield, IL 62710"),

        // Psych Clinic patients
        ("Emily", "Davis", "1995-04-17", "F", "(555) 234-5679", "edavis95@email.com", "963 Cherry Lane, Springfield, IL 62711"),
        ("Michael", "Thompson", "1988-10-09", "M", "(555) 345-6780", "mthompson@email.com", "174 Peach Court, Springfield, IL 62712"),
        ("Sarah", "Lee", "2001-01-25", "F", "(555) 456-7891", "slee2001@email.com", "285 Apple Way, Springfield, IL 62713"),
    ];

    let mut patient_ids: Vec<i64> = Vec::new();

    for (first, last, dob, sex, phone, email, address) in patients_data {
        conn.execute(
            "INSERT INTO patients (first_name, last_name, dob, sex, phone, email, address)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![first, last, dob, sex, phone, email, address],
        )?;
        patient_ids.push(conn.last_insert_rowid());
    }

    // Add some diagnoses for variety
    // Marcus Williams - Diabetes, Hypertension
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Type 2 Diabetes Mellitus', 'E11.9', 'active', 'Endocrine')",
        params![patient_ids[0]],
    )?;
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Essential Hypertension', 'I10', 'active', 'Cardiovascular')",
        params![patient_ids[0]],
    )?;

    // Jennifer Garcia - Asthma
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Mild Persistent Asthma', 'J45.30', 'active', 'Respiratory')",
        params![patient_ids[1]],
    )?;

    // Thomas Anderson - Chronic pain
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Chronic Low Back Pain', 'M54.5', 'active', 'Musculoskeletal')",
        params![patient_ids[4]],
    )?;
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Lumbar Radiculopathy', 'M54.16', 'active', 'Musculoskeletal')",
        params![patient_ids[4]],
    )?;

    // Maria Rodriguez - Fibromyalgia
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Fibromyalgia', 'M79.7', 'active', 'Musculoskeletal')",
        params![patient_ids[5]],
    )?;

    // Patricia Brown - CHF, COPD (inpatient)
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Heart Failure with reduced EF', 'I50.22', 'active', 'Cardiovascular')",
        params![patient_ids[7]],
    )?;
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'COPD with acute exacerbation', 'J44.1', 'active', 'Respiratory')",
        params![patient_ids[7]],
    )?;

    // Emily Davis - Depression, Anxiety
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Major Depressive Disorder, recurrent', 'F33.1', 'active', 'Psychiatric')",
        params![patient_ids[10]],
    )?;
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Generalized Anxiety Disorder', 'F41.1', 'active', 'Psychiatric')",
        params![patient_ids[10]],
    )?;

    // Michael Thompson - Bipolar
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'Bipolar I Disorder', 'F31.9', 'active', 'Psychiatric')",
        params![patient_ids[11]],
    )?;

    // Sarah Lee - ADHD
    conn.execute(
        "INSERT INTO diagnoses (patient_id, name, icd_code, status, category) VALUES (?1, 'ADHD, Combined Type', 'F90.2', 'active', 'Psychiatric')",
        params![patient_ids[12]],
    )?;

    // Create patient lists
    // 1. Resident Clinic (default)
    let resident_list = PatientList {
        id: None,
        user_id,
        name: "Resident Clinic".to_string(),
        description: Some("General outpatient clinic patients".to_string()),
        color: Some("#3B82F6".to_string()),
        icon: Some("fa-stethoscope".to_string()),
        is_default: true,
        sort_order: 0,
    };
    let resident_list_id = create_patient_list(conn, &resident_list)?;

    // Add patients 0-3 to resident clinic
    for i in 0..4 {
        add_patient_to_list(conn, resident_list_id, patient_ids[i], None)?;
    }

    // 2. Pain Clinic
    let pain_list = PatientList {
        id: None,
        user_id,
        name: "Pain Clinic".to_string(),
        description: Some("Chronic pain management patients".to_string()),
        color: Some("#EF4444".to_string()),
        icon: Some("fa-hand-dots".to_string()),
        is_default: false,
        sort_order: 1,
    };
    let pain_list_id = create_patient_list(conn, &pain_list)?;

    // Add pain clinic patients 4-6
    for i in 4..7 {
        add_patient_to_list(conn, pain_list_id, patient_ids[i], None)?;
    }

    // 3. Yellow Team
    let yellow_list = PatientList {
        id: None,
        user_id,
        name: "Yellow Team".to_string(),
        description: Some("Inpatient medicine team".to_string()),
        color: Some("#F59E0B".to_string()),
        icon: Some("fa-hospital".to_string()),
        is_default: false,
        sort_order: 2,
    };
    let yellow_list_id = create_patient_list(conn, &yellow_list)?;

    // Add inpatient patients 7-9
    for i in 7..10 {
        add_patient_to_list(conn, yellow_list_id, patient_ids[i], None)?;
    }

    // 4. Psych Clinic
    let psych_list = PatientList {
        id: None,
        user_id,
        name: "Psych Clinic".to_string(),
        description: Some("Psychiatry outpatient clinic".to_string()),
        color: Some("#8B5CF6".to_string()),
        icon: Some("fa-brain".to_string()),
        is_default: false,
        sort_order: 3,
    };
    let psych_list_id = create_patient_list(conn, &psych_list)?;

    // Add psych patients 10-12
    for i in 10..13 {
        add_patient_to_list(conn, psych_list_id, patient_ids[i], None)?;
    }

    Ok(())
}
