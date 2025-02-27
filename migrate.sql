CREATE TABLE IF NOT EXISTS Status(
    id INTEGER PRIMARY KEY ASC,
    name TEXT
);

CREATE TABLE IF NOT EXISTS Faculty(
    id INTEGER PRIMARY KEY ASC,
    name TEXT
);

CREATE TABLE IF NOT EXISTS Program(
    id INTEGER PRIMARY KEY ASC,
    name TEXT
);

CREATE TABLE IF NOT EXISTS Student(
    id VARCHAR(20) NOT NULL PRIMARY KEY,
    name TEXT,
    dob TEXT,
    phone TEXT,
    address TEXT,
    email TEXT,
    status INTEGER REFERENCES Status(id),
    gender INTEGER,
    faculty INTEGER REFERENCES Faculty(id),
    enrolled_year INTEGER CHECK(enrolled_year > 0),
    program INTEGER REFERENCES Program(id),
    created_time INTEGER DEFAULT (UNIXEPOCH())
);

-- BEGIN;
--     INSERT INTO Student_new (id, name, dob, phone, address, email, status, gender, faculty, enrolled_year, program)
--     SELECT id, name, dob, phone, address, email, status, gender, faculty, enrolled_year, program FROM Student;
-- COMMIT;
-- DROP TABLE IF EXISTS Student;
-- ALTER TABLE Student_new RENAME TO Student;

-- initialize database with some data if empty
BEGIN;
    INSERT OR IGNORE INTO Faculty(id, name) VALUES(1, 'Khoa Luật'),
                                        (2, 'Khoa Tiếng Anh thương mại'),
                                        (3, 'Khoa Tiếng Nhật'),
                                        (4, 'Khoa Tiếng Pháp');
    INSERT OR IGNORE INTO Status(id, name) VALUES(1, 'Đang học'),
                                       (2, 'Đã tốt nghiệp'),
                                       (3, 'Đã thôi học'),
                                       (4, 'Tạm dừng học');
    INSERT OR IGNORE INTO Program(id, name) VALUES(1, 'Thường'),
                                        (2, 'Chất lượng cao');
COMMIT;

