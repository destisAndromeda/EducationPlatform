use rusqlite::{
    Connection,
    Result,
    params,
};
use std::collections::HashMap;
use serde_json;

#[derive(Debug)]
pub struct Question {
    pub id: i64,
    pub subject:  String,
    pub section:  String,
    pub question: String,
    pub options:  HashMap<String, String>,
    pub answer:   Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Answer {
    pub id: i64,
    pub subject: String,
    pub section: String,
    pub question: String,
    pub options: HashMap<String, String>,
    pub answer: Vec<String>,
}

#[derive(serde::Serialize, PartialEq, Debug)]
pub enum Flag {
    Green,
    Yellow,
    Red,    
} 

impl Flag {
    pub fn new() -> Self {
        Self::Red
    }

    pub fn get_flag(q: &Vec<String>, a: &Vec<String>) -> Self {
        let mut result = Self::new();

        let mut onetime = Self::new();

        a.iter().for_each(|a| {
            if !q.contains(a) {
                result = Self::Red;
            }

            if q.contains(a) {
                result = Self::Green;
                onetime = Self::Green;
            }

            if !q.contains(a) && onetime == Self::Green {
                result = Self::Yellow;
            }
        });

        result
    }
}

#[derive(serde::Serialize, Debug)]
pub struct Assussment {
    pub id: i64,
    pub flag: Flag,
} 

#[derive(serde::Serialize)]
pub struct FormatedQuestions {
    id: i64,
    subject: String,
    section: String,
    question: String,
    options: HashMap<String, String>,
}

impl FormatedQuestions {
    pub fn new(questions: &Vec<Question>) -> Vec<Self> {
        questions
            .iter()
            .map(|q| {
                Self {
                    id: q.id,
                    subject: q.subject.clone(),
                    section: q.section.clone(),
                    question: q.question.clone(),
                    options: q.options.clone(),
                }
            })
            .collect()
    }
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
    
        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS questions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                subject  TEXT NOT NULL,
                section  TEXT NOT NULL,
                question TEXT NOT NULL,
                options  TEXT NOT NULL,
                answer   TEXT NOT NULL
            );
        ")?;

        Ok( Self { conn } )
    }

    pub fn insert_question(&mut self, question: &Question) -> Result<()> {
        let tx = self.conn.transaction()?;

        let options_json = serde_json::to_string(&question.options).unwrap();
        let answer_json  = serde_json::to_string(&question.answer).unwrap();
 
        tx.execute("
            INSERT INTO questions (subject, section, question, options, answer)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &question.subject,
                &question.section,
                &question.question,
                options_json,
                answer_json,
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn get_by_subject(&self, subject: &str) -> Result<Vec<Question>> {
        let mut stmt = self.conn.prepare("
            SELECT id, subject, section, question, options, answer
            FROM questions WHERE subject = ?1
        ")?;

        let questions = stmt.query_map(params![subject], |row| {
            let options_json: String = row.get(4)?;
            let answer_json:  String = row.get(5)?;

            Ok( Question {
                id:       row.get(0)?,
                subject:  row.get(1)?,
                section:  row.get(2)?,
                question: row.get(3)?,
                options:  serde_json::from_str(&options_json).unwrap_or_default(),
                answer:   serde_json::from_str(&answer_json).unwrap_or_default(),
            })
        })?
        .filter_map(|r| r.ok())
        .collect();



        Ok(questions)
    }

    pub fn get_subjects(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("
            SELECT DISTINCT subject FROM questions ORDER BY subject
        ")?;

        let subjects = stmt.query_map(params![], |row| {
            Ok(row.get(0)?)
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(subjects)
    }

    pub fn get_random_questions(&self, subject: &str) -> Result<Vec<Question>> {
        let mut stmt = self.conn.prepare("
            SELECT * FROM questions
            WHERE subject = ?
            ORDER BY RANDOM()
            LIMIT 30;
        ")?;

        let random_questions = stmt.query_map(params![subject], |row| {
            let options_json: String = row.get(4)?;
            let answer_json: String  = row.get(5)?;
            
            Ok( Question {
                id: row.get(0)?,
                subject: row.get(1)?,
                section: row.get(2)?,
                question: row.get(3)?,
                options: serde_json::from_str(&options_json).unwrap_or_default(),
                answer: serde_json::from_str(&answer_json).unwrap_or_default(),
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(random_questions)
    }
}