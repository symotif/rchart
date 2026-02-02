// disable the command prompt window that would normally pop up if someone is on windows running a bundled app
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod db;

use db::{DbState, Patient, Appointment, AppointmentWithPatient, PatientFullData, Encounter, User, UserFullData, UserSettings, SearchResult, Prescription, Allergy, Vaccination, SocialHistory, FamilyHistory};
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
            // Patient list commands
            db_get_patient_lists,
            db_get_patient_list,
            db_create_patient_list,
            db_update_patient_list,
            db_delete_patient_list,
            db_get_patients_in_list,
            db_add_patient_to_list,
            db_remove_patient_from_list,
            db_update_list_columns,
            db_seed_patient_lists,
            // Search commands
            db_global_search,
            db_search_patient_data,
            db_quick_search_patients,
            // Prescription commands
            db_create_prescriptions,
            db_get_prescriptions,
            // History CRUD commands
            db_create_allergy,
            db_update_allergy,
            db_delete_allergy,
            db_create_vaccination,
            db_update_vaccination,
            db_delete_vaccination,
            db_create_social_history,
            db_update_social_history,
            db_delete_social_history,
            db_create_family_history,
            db_update_family_history,
            db_delete_family_history,
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

    // Seed test patients - 20 diverse, realistic patients
    let test_patients = vec![
        Patient {
            id: None,
            first_name: "Marcus".to_string(),
            last_name: "Thompson".to_string(),
            dob: "1958-03-14".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("1847 Woodward Ave, Detroit, MI 48201".to_string()),
            phone: Some("313-555-0142".to_string()),
            email: Some("mthompson58@gmail.com".to_string()),
            photo_url: None,
            ai_summary: Some("67yo male with HTN, T2DM, and CKD stage 3. Recently hospitalized for CHF exacerbation. On multiple antihypertensives. Adherent to medications.".to_string()),
            preferred_pharmacy: Some("CVS Pharmacy - 2100 Woodward Ave, Detroit".to_string()),
            insurance_provider: Some("Medicare".to_string()),
            insurance_policy_number: Some("1EG4-TE5-MK72".to_string()),
            insurance_group_number: None,
        },
        Patient {
            id: None,
            first_name: "Sarah".to_string(),
            last_name: "Okonkwo".to_string(),
            dob: "1992-08-23".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("4521 Cass Ave Apt 3B, Detroit, MI 48201".to_string()),
            phone: Some("313-555-0287".to_string()),
            email: Some("sarah.okonkwo@email.com".to_string()),
            photo_url: None,
            ai_summary: Some("32yo female with generalized anxiety disorder and migraines. Well-controlled on current regimen. Works as a nurse, reports high stress levels.".to_string()),
            preferred_pharmacy: Some("Walgreens - 4600 Cass Ave, Detroit".to_string()),
            insurance_provider: Some("Blue Cross Blue Shield".to_string()),
            insurance_policy_number: Some("XYZ123456789".to_string()),
            insurance_group_number: Some("GRP-HENRY-001".to_string()),
        },
        Patient {
            id: None,
            first_name: "Robert".to_string(),
            last_name: "Kowalski".to_string(),
            dob: "1945-11-02".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("8834 Joseph Campau St, Hamtramck, MI 48212".to_string()),
            phone: Some("313-555-0391".to_string()),
            email: None,
            photo_url: None,
            ai_summary: Some("79yo male with COPD on home O2, AFib on warfarin, and BPH. Former 40 pack-year smoker, quit 10 years ago. Lives alone, daughter checks in daily.".to_string()),
            preferred_pharmacy: Some("Rite Aid - 9000 Joseph Campau, Hamtramck".to_string()),
            insurance_provider: Some("Medicare".to_string()),
            insurance_policy_number: Some("1AB2-CD3-EF45".to_string()),
            insurance_group_number: None,
        },
        Patient {
            id: None,
            first_name: "Jennifer".to_string(),
            last_name: "Martinez-Reyes".to_string(),
            dob: "1987-05-19".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("2901 W Vernor Hwy, Detroit, MI 48216".to_string()),
            phone: Some("313-555-0445".to_string()),
            email: Some("jmartinez.reyes@outlook.com".to_string()),
            photo_url: None,
            ai_summary: Some("37yo female with lupus (SLE) and secondary Sjogren's. On hydroxychloroquine and methotrexate. Last flare 6 months ago. Planning pregnancy, needs rheum consult.".to_string()),
            preferred_pharmacy: Some("CVS Pharmacy - 3000 W Vernor Hwy, Detroit".to_string()),
            insurance_provider: Some("Aetna".to_string()),
            insurance_policy_number: Some("W12345678".to_string()),
            insurance_group_number: Some("0054321".to_string()),
        },
        Patient {
            id: None,
            first_name: "David".to_string(),
            last_name: "Washington".to_string(),
            dob: "1975-09-30".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("15620 Plymouth Rd, Detroit, MI 48227".to_string()),
            phone: Some("313-555-0512".to_string()),
            email: Some("dwash75@yahoo.com".to_string()),
            photo_url: None,
            ai_summary: Some("49yo male with bipolar I disorder, stable on lithium and quetiapine. History of 2 psychiatric hospitalizations, last one 5 years ago. Works part-time, good family support.".to_string()),
            preferred_pharmacy: Some("Walgreens - 15700 Plymouth Rd, Detroit".to_string()),
            insurance_provider: Some("Medicaid".to_string()),
            insurance_policy_number: Some("MED987654321".to_string()),
            insurance_group_number: None,
        },
        Patient {
            id: None,
            first_name: "Linda".to_string(),
            last_name: "Patel".to_string(),
            dob: "1969-01-08".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("42850 Garfield Rd, Clinton Township, MI 48038".to_string()),
            phone: Some("586-555-0623".to_string()),
            email: Some("lindapatel@gmail.com".to_string()),
            photo_url: None,
            ai_summary: Some("56yo female with T2DM, hypothyroidism, and osteoporosis. A1c trending up despite metformin, may need additional agent. Due for DEXA scan.".to_string()),
            preferred_pharmacy: Some("Costco Pharmacy - 35400 S Gratiot Ave, Clinton Twp".to_string()),
            insurance_provider: Some("United Healthcare".to_string()),
            insurance_policy_number: Some("UHC-998877665".to_string()),
            insurance_group_number: Some("GRP-AUTO-500".to_string()),
        },
        Patient {
            id: None,
            first_name: "Anthony".to_string(),
            last_name: "Russo".to_string(),
            dob: "1982-12-25".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("1955 E Jefferson Ave Unit 1201, Detroit, MI 48207".to_string()),
            phone: Some("313-555-0734".to_string()),
            email: Some("anthony.russo@lawfirm.com".to_string()),
            photo_url: None,
            ai_summary: Some("42yo male with GERD and obesity (BMI 34). Interested in weight loss options. Stress-related symptoms, works long hours as attorney. Sleep study pending.".to_string()),
            preferred_pharmacy: Some("Henry Ford Pharmacy - 2799 W Grand Blvd, Detroit".to_string()),
            insurance_provider: Some("Cigna".to_string()),
            insurance_policy_number: Some("CIG-456789012".to_string()),
            insurance_group_number: Some("GRP-LAW-200".to_string()),
        },
        Patient {
            id: None,
            first_name: "Mary".to_string(),
            last_name: "O'Brien".to_string(),
            dob: "1952-07-04".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("19500 Mack Ave, Grosse Pointe Woods, MI 48236".to_string()),
            phone: Some("313-555-0856".to_string()),
            email: Some("mobrien52@aol.com".to_string()),
            photo_url: None,
            ai_summary: Some("72yo female with breast cancer (ER+, s/p lumpectomy 2022), now on anastrozole. Also HTN and osteoarthritis. Active, plays tennis twice weekly.".to_string()),
            preferred_pharmacy: Some("Walgreens - 19700 Mack Ave, Grosse Pointe Woods".to_string()),
            insurance_provider: Some("Medicare".to_string()),
            insurance_policy_number: Some("1HJ7-KL8-MN90".to_string()),
            insurance_group_number: None,
        },
        Patient {
            id: None,
            first_name: "Kevin".to_string(),
            last_name: "Nguyen".to_string(),
            dob: "1995-04-17".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("2455 W Chicago Blvd, Detroit, MI 48206".to_string()),
            phone: Some("313-555-0967".to_string()),
            email: Some("knguyen95@gmail.com".to_string()),
            photo_url: None,
            ai_summary: Some("29yo male with ADHD and seasonal allergies. Stable on Adderall XR. Recent college graduate, starting new job. Needs medication refill.".to_string()),
            preferred_pharmacy: Some("CVS Pharmacy - 2500 W Chicago Blvd, Detroit".to_string()),
            insurance_provider: Some("Blue Cross Blue Shield".to_string()),
            insurance_policy_number: Some("BCB-112233445".to_string()),
            insurance_group_number: Some("GRP-TECH-100".to_string()),
        },
        Patient {
            id: None,
            first_name: "Patricia".to_string(),
            last_name: "Jackson".to_string(),
            dob: "1963-10-11".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("8901 E Seven Mile Rd, Detroit, MI 48234".to_string()),
            phone: Some("313-555-1078".to_string()),
            email: Some("pjackson63@email.com".to_string()),
            photo_url: None,
            ai_summary: Some("61yo female with fibromyalgia, depression, and chronic low back pain. On duloxetine and gabapentin. Participates in PT. Disability application pending.".to_string()),
            preferred_pharmacy: Some("Rite Aid - 9000 E Seven Mile Rd, Detroit".to_string()),
            insurance_provider: Some("Medicaid".to_string()),
            insurance_policy_number: Some("MED-223344556".to_string()),
            insurance_group_number: None,
        },
        Patient {
            id: None,
            first_name: "Mohammed".to_string(),
            last_name: "Al-Rashid".to_string(),
            dob: "1978-02-28".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("6750 Schaefer Rd, Dearborn, MI 48126".to_string()),
            phone: Some("313-555-1189".to_string()),
            email: Some("malrashid@business.com".to_string()),
            photo_url: None,
            ai_summary: Some("46yo male with T2DM, HTN, and hyperlipidemia. A1c at goal on metformin/glipizide. Family history of early CAD. Due for stress test.".to_string()),
            preferred_pharmacy: Some("CVS Pharmacy - 6800 Schaefer Rd, Dearborn".to_string()),
            insurance_provider: Some("Aetna".to_string()),
            insurance_policy_number: Some("AET-667788990".to_string()),
            insurance_group_number: Some("GRP-SMB-300".to_string()),
        },
        Patient {
            id: None,
            first_name: "Elizabeth".to_string(),
            last_name: "Taylor".to_string(),
            dob: "1990-06-15".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("3200 E Grand Blvd, Detroit, MI 48202".to_string()),
            phone: Some("313-555-1290".to_string()),
            email: Some("etaylor90@proton.me".to_string()),
            photo_url: None,
            ai_summary: Some("34yo female with Crohn's disease on Humira, doing well. Also has iron deficiency anemia. Vegetarian diet. Last colonoscopy 2023 showed mucosal healing.".to_string()),
            preferred_pharmacy: Some("Specialty Pharmacy - Henry Ford Health".to_string()),
            insurance_provider: Some("United Healthcare".to_string()),
            insurance_policy_number: Some("UHC-778899001".to_string()),
            insurance_group_number: Some("GRP-EDU-150".to_string()),
        },
        Patient {
            id: None,
            first_name: "William".to_string(),
            last_name: "Henderson".to_string(),
            dob: "1955-08-20".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("27300 Dequindre Rd, Warren, MI 48092".to_string()),
            phone: Some("586-555-1345".to_string()),
            email: Some("whenderson55@gmail.com".to_string()),
            photo_url: None,
            ai_summary: Some("69yo male with Parkinson's disease, depression, and REM sleep behavior disorder. On carbidopa/levodopa TID. Some motor fluctuations, considering DBS referral.".to_string()),
            preferred_pharmacy: Some("Walgreens - 27500 Dequindre Rd, Warren".to_string()),
            insurance_provider: Some("Medicare".to_string()),
            insurance_policy_number: Some("1QR2-ST3-UV45".to_string()),
            insurance_group_number: None,
        },
        Patient {
            id: None,
            first_name: "Angela".to_string(),
            last_name: "Foster".to_string(),
            dob: "1988-11-09".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("5600 Conner St, Detroit, MI 48213".to_string()),
            phone: Some("313-555-1456".to_string()),
            email: Some("afoster88@yahoo.com".to_string()),
            photo_url: None,
            ai_summary: Some("36yo female G3P2 at 28 weeks gestation. Gestational diabetes, diet-controlled. History of preeclampsia with last pregnancy. High-risk OB following.".to_string()),
            preferred_pharmacy: Some("CVS Pharmacy - 5700 Conner St, Detroit".to_string()),
            insurance_provider: Some("Medicaid".to_string()),
            insurance_policy_number: Some("MED-445566778".to_string()),
            insurance_group_number: None,
        },
        Patient {
            id: None,
            first_name: "James".to_string(),
            last_name: "Kim".to_string(),
            dob: "1972-03-03".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("1200 Brush St Apt 805, Detroit, MI 48226".to_string()),
            phone: Some("313-555-1567".to_string()),
            email: Some("jkim72@architect.com".to_string()),
            photo_url: None,
            ai_summary: Some("52yo male with HIV (well-controlled, undetectable VL), hyperlipidemia, and prediabetes. On Biktarvy. Excellent adherence. Due for colonoscopy screening.".to_string()),
            preferred_pharmacy: Some("Walgreens Specialty - 1300 Brush St, Detroit".to_string()),
            insurance_provider: Some("Cigna".to_string()),
            insurance_policy_number: Some("CIG-889900112".to_string()),
            insurance_group_number: Some("GRP-ARCH-050".to_string()),
        },
        Patient {
            id: None,
            first_name: "Dorothy".to_string(),
            last_name: "Williams".to_string(),
            dob: "1940-12-18".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("18900 Livernois Ave, Detroit, MI 48221".to_string()),
            phone: Some("313-555-1678".to_string()),
            email: None,
            photo_url: None,
            ai_summary: Some("84yo female with dementia (moderate), HTN, and recurrent UTIs. Lives with daughter who is primary caregiver. Recently started on memantine. DNR/DNI.".to_string()),
            preferred_pharmacy: Some("Rite Aid - 19000 Livernois Ave, Detroit".to_string()),
            insurance_provider: Some("Medicare".to_string()),
            insurance_policy_number: Some("1WX2-YZ3-AB45".to_string()),
            insurance_group_number: None,
        },
        Patient {
            id: None,
            first_name: "Christopher".to_string(),
            last_name: "Brown".to_string(),
            dob: "1999-07-22".to_string(),
            sex: "M".to_string(),
            gender: Some("NB".to_string()),
            address: Some("4400 John R St, Detroit, MI 48201".to_string()),
            phone: Some("313-555-1789".to_string()),
            email: Some("cbrown99@university.edu".to_string()),
            photo_url: None,
            ai_summary: Some("25yo patient (they/them) with gender dysphoria on HRT, depression, and asthma. Followed by endocrine for testosterone therapy. Doing well overall.".to_string()),
            preferred_pharmacy: Some("CVS Pharmacy - 4500 John R St, Detroit".to_string()),
            insurance_provider: Some("Blue Cross Blue Shield".to_string()),
            insurance_policy_number: Some("BCB-990011223".to_string()),
            insurance_group_number: Some("GRP-UNI-025".to_string()),
        },
        Patient {
            id: None,
            first_name: "Nancy".to_string(),
            last_name: "Garcia".to_string(),
            dob: "1965-05-27".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("7800 Vernor Hwy, Detroit, MI 48209".to_string()),
            phone: Some("313-555-1890".to_string()),
            email: Some("ngarcia65@hotmail.com".to_string()),
            photo_url: None,
            ai_summary: Some("59yo female with rheumatoid arthritis on methotrexate/Enbrel, HTN, and chronic hepatitis C (SVR 2021). Joint pain well-controlled. Spanish interpreter needed.".to_string()),
            preferred_pharmacy: Some("Walgreens - 7900 Vernor Hwy, Detroit".to_string()),
            insurance_provider: Some("Aetna".to_string()),
            insurance_policy_number: Some("AET-112233445".to_string()),
            insurance_group_number: Some("GRP-RET-100".to_string()),
        },
        Patient {
            id: None,
            first_name: "Thomas".to_string(),
            last_name: "Murphy".to_string(),
            dob: "1980-09-05".to_string(),
            sex: "M".to_string(),
            gender: Some("M".to_string()),
            address: Some("22100 Michigan Ave, Dearborn, MI 48124".to_string()),
            phone: Some("313-555-1901".to_string()),
            email: Some("tmurphy80@ford.com".to_string()),
            photo_url: None,
            ai_summary: Some("44yo male with alcohol use disorder (in recovery, 2 years sober), PTSD, and chronic knee pain s/p ACL repair. Attends AA regularly. On naltrexone.".to_string()),
            preferred_pharmacy: Some("CVS Pharmacy - 22200 Michigan Ave, Dearborn".to_string()),
            insurance_provider: Some("United Healthcare".to_string()),
            insurance_policy_number: Some("UHC-334455667".to_string()),
            insurance_group_number: Some("GRP-AUTO-250".to_string()),
        },
        Patient {
            id: None,
            first_name: "Samantha".to_string(),
            last_name: "Lee".to_string(),
            dob: "2010-01-30".to_string(),
            sex: "F".to_string(),
            gender: Some("F".to_string()),
            address: Some("3100 W Outer Dr, Detroit, MI 48221".to_string()),
            phone: Some("313-555-2012".to_string()),
            email: Some("slee.parent@email.com".to_string()),
            photo_url: None,
            ai_summary: Some("14yo female with Type 1 DM on insulin pump, celiac disease, and ADHD. A1c 7.2%, improved from 8.1%. Active in sports, does well in school. Parent contact for communications.".to_string()),
            preferred_pharmacy: Some("Walgreens - 3200 W Outer Dr, Detroit".to_string()),
            insurance_provider: Some("Blue Cross Blue Shield".to_string()),
            insurance_policy_number: Some("BCB-556677889".to_string()),
            insurance_group_number: Some("GRP-FAM-075".to_string()),
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

// ============ Patient List Commands ============

#[tauri::command]
fn db_get_patient_lists(state: State<DbState>, user_id: i64) -> Result<Vec<db::PatientList>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_patient_lists_for_user(&conn, user_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_get_patient_list(state: State<DbState>, list_id: i64) -> Result<Option<db::PatientListWithPatients>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_patient_list_with_patients(&conn, list_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_create_patient_list(state: State<DbState>, list: db::PatientList) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_patient_list(&conn, &list).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_patient_list(state: State<DbState>, list: db::PatientList) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_patient_list(&conn, &list).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_delete_patient_list(state: State<DbState>, list_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_patient_list(&conn, list_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_get_patients_in_list(state: State<DbState>, list_id: i64) -> Result<Vec<db::Patient>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_patients_in_list(&conn, list_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_add_patient_to_list(state: State<DbState>, list_id: i64, patient_id: i64, notes: Option<String>) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::add_patient_to_list(&conn, list_id, patient_id, notes.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_remove_patient_from_list(state: State<DbState>, list_id: i64, patient_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::remove_patient_from_list(&conn, list_id, patient_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_list_columns(state: State<DbState>, list_id: i64, columns: Vec<db::PatientListColumn>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_list_columns(&conn, list_id, &columns).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_seed_patient_lists(state: State<DbState>, user_id: i64) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::seed_patient_lists(&conn, user_id).map_err(|e| e.to_string())?;
    Ok("Patient lists seeded successfully".to_string())
}

// ============ Search Commands ============

#[tauri::command]
fn db_global_search(state: State<DbState>, query: String, limit: Option<i64>) -> Result<Vec<SearchResult>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::global_search(&conn, &query, limit.unwrap_or(20)).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_search_patient_data(state: State<DbState>, patient_id: i64, query: String, limit: Option<i64>) -> Result<Vec<SearchResult>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::search_patient_data(&conn, patient_id, &query, limit.unwrap_or(20)).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_quick_search_patients(state: State<DbState>, query: String, limit: Option<i64>) -> Result<Vec<Patient>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::quick_search_patients(&conn, &query, limit.unwrap_or(50)).map_err(|e| e.to_string())
}

// ============ Prescription Commands ============

#[tauri::command]
fn db_create_prescriptions(state: State<DbState>, prescriptions: Vec<Prescription>) -> Result<Vec<i64>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_prescriptions_batch(&conn, &prescriptions).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_get_prescriptions(state: State<DbState>, patient_id: i64) -> Result<Vec<Prescription>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_prescriptions_for_patient(&conn, patient_id).map_err(|e| e.to_string())
}

// ============ History CRUD Commands ============

// Allergy commands
#[tauri::command]
fn db_create_allergy(state: State<DbState>, allergy: Allergy) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_allergy(&conn, &allergy).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_allergy(state: State<DbState>, allergy: Allergy) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_allergy(&conn, &allergy).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_delete_allergy(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_allergy(&conn, id).map_err(|e| e.to_string())
}

// Vaccination commands
#[tauri::command]
fn db_create_vaccination(state: State<DbState>, vaccination: Vaccination) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_vaccination(&conn, &vaccination).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_vaccination(state: State<DbState>, vaccination: Vaccination) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_vaccination(&conn, &vaccination).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_delete_vaccination(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_vaccination(&conn, id).map_err(|e| e.to_string())
}

// Social History commands
#[tauri::command]
fn db_create_social_history(state: State<DbState>, history: SocialHistory) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_social_history(&conn, &history).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_social_history(state: State<DbState>, history: SocialHistory) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_social_history(&conn, &history).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_delete_social_history(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_social_history(&conn, id).map_err(|e| e.to_string())
}

// Family History commands
#[tauri::command]
fn db_create_family_history(state: State<DbState>, history: FamilyHistory) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_family_history(&conn, &history).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_update_family_history(state: State<DbState>, history: FamilyHistory) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_family_history(&conn, &history).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_delete_family_history(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_family_history(&conn, id).map_err(|e| e.to_string())
}
