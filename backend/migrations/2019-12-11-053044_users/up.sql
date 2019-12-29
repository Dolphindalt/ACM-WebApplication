CREATE TABLE passwords (
  password_id int NOT NULL AUTO_INCREMENT,
  password varchar(255) NOT NULL,
  verification_code varchar(8),
  PRIMARY KEY(password_id)
);

CREATE TABLE user_types (
  user_type_id tinyint NOT NULL AUTO_INCREMENT,
  name varchar(20) NOT NULL,
  description varchar(180),
  is_admin boolean NOT NULL,
  PRIMARY KEY(user_type_id)
);

INSERT INTO user_types (name, description, is_admin)
VALUES ("Member", "A member of the student ACM chapter.", false);
INSERT INTO user_types (name, description, is_admin)
VALUES ("Secretary", "The one who controls information.", true);
INSERT INTO user_types (name, description, is_admin)
VALUES ("Treasurer", "The financial magician.", true);
INSERT INTO user_types (name, description, is_admin)
VALUES ("Vice President", "Next in line or someone to pass up?", true);
INSERT INTO user_types (name, description, is_admin)
VALUES ("President", "The defacto leader of the ACM student chapter.", true);
INSERT INTO user_types (name, description, is_admin)
VALUES ("Mascot", "In honor of Josh Baldwin and the legacy of his predecessors.", false);


CREATE TABLE users(
    user_id int NOT NULL AUTO_INCREMENT,
    password_id int NOT NULL,
    user_type tinyint NOT NULL,
    first_name varchar(20) NOT NULL,
    last_name varchar(20) NOT NULL,
    email varchar(30) NOT NULL UNIQUE,
    points float NOT NULL,
    FOREIGN KEY(password_id) REFERENCES passwords(password_id),
    FOREIGN KEY(user_type) REFERENCES user_types(user_type_id),
    PRIMARY KEY(user_id)
);

INSERT INTO passwords (password, verification_code)
VALUES ("3b2d6c9d79b3996ea7ab4b4ec13edb9060856d67ec4e0575d45823aee7610288", null);
INSERT INTO passwords (password, verification_code)
VALUES ("3b2d6c9d79b3996ea7ab4b4ec13edb9060856d67ec4e0575d45823aee7610288", "varcode");
INSERT INTO users (password_id, user_type, first_name, last_name, email, points)
VALUES (1, 5, "Jeff", "Braun", "jbraun@mtech.edu", 0.0);
INSERT INTO users (password_id, user_type, first_name, last_name, email, points)
VALUES (2, 1, "Frank", "Ackerman", "fackerman@mtech.edu", 0.0);

CREATE TABLE files(
  file_id int NOT NULL AUTO_INCREMENT,
  uploader int NOT NULL,
  audience tinyint NOT NULL,
  file_name varchar(30) NOT NULL UNIQUE,
  description varchar(140) NOT NULL,
  FOREIGN KEY(uploader) REFERENCES users(user_id),
  FOREIGN KEY(audience) REFERENCES user_types(user_type_id),
  PRIMARY KEY(file_id)
);

INSERT INTO files (uploader, audience, file_name, description)
VALUES (1, 1, "bruh.jpg", "A bruh for testing.");

CREATE TABLE event_types (
    event_type_id tinyint NOT NULL AUTO_INCREMENT,
    name varchar(40) NOT NULL,
    description varchar(140) NOT NULL,
    default_points float NOT NULL,
    PRIMARY KEY(event_type_id)
);

INSERT INTO event_types (name, description, default_points)
VALUES ('Meeting', 'General club meetings for all members', 125.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('LAN Party', 'LAN Party, games, and pizza', 50.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('Programming Comp', 'Programming competitions', 150.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('Programming Prac', 'Practice for competitions', 75.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('Industry Trip', 'Going out and seeing companies', 200.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('Homecoming', 'Attendance for Homecoming events', 100.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('Club Rush', 'Club Rush help', 100.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('ASMT Meeting', 'Attendance for an ASMT proposal meeting', 75.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('DnD Night', 'Dungeons and Dragons game night', 50.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('Board Meeting', 'Board Meeting for Officers', 75.0);

INSERT INTO event_types (name, description, default_points)
VALUES ('Study Group', 'Time for students to help each other with homework', 100.0);

CREATE TABLE events (
    event_id int NOT NULL AUTO_INCREMENT,
    coordinator_id int,
    event_type_id tinyint NOT NULL,
    name varchar(30) NOT NULL,
    additional_info varchar(140),
    location varchar(50) NOT NULL,
    event_time timestamp NOT NULL,
    points float NOT NULL,
    FOREIGN KEY(coordinator_id) REFERENCES users(user_id),
    FOREIGN KEY(event_type_id) REFERENCES event_types(event_type_id),
    PRIMARY KEY(event_id)
);

INSERT INTO events (coordinator_id, event_type_id, name, additional_info, location, event_time, points)
VALUES (1, 2, "LAN Party Test", "A test of the LAN party event!", "Museum lab", "2007-04-05T14:30:30", 0.0);

INSERT INTO events (coordinator_id, event_type_id, name, additional_info, location, event_time, points)
VALUES (1, 2, "Bungalo Dance Party Night", "We are going to boogie woogie in the bungalo!", "Greens Apartments", "2030-04-05T14:30:30", 50.0);

INSERT INTO events (coordinator_id, event_type_id, name, additional_info, location, event_time, points)
VALUES (1, 2, "Bungalo Dance Party Night 2", "Ameno ameno latire latiremo ameno", "Greens Apartments", "2031-04-05T14:30:30", 100.0);

CREATE TABLE event_files(
  dummy_id int NOT NULL AUTO_INCREMENT,
  file_id int NOT NULL,
  event_id int NOT NULL,
  additional_info varchar(140),
  FOREIGN KEY(event_id) REFERENCES events(event_id),
  PRIMARY KEY(dummy_id)
);

INSERT INTO event_files (file_id, event_id, additional_info)
VALUES (1, 1, "Additional info related to the event.");

INSERT INTO event_files (file_id, event_id, additional_info)
VALUES (1, 3, "A dummy file 1.");

INSERT INTO event_files (file_id, event_id, additional_info)
VALUES (1, 3, "A dummy file 2.");

CREATE TABLE user_attendences(
  user_attendence_id int NOT NULL AUTO_INCREMENT,
  user_id int NOT NULL,
  event_id int NOT NULL,
  given_points float NOT NULL,
  additional_info varchar(140),
  FOREIGN KEY(user_id) REFERENCES users(user_id),
  FOREIGN KEY(event_id) REFERENCES events(event_id),
  PRIMARY KEY(user_attendence_id)
);

CREATE TABLE fee_types(
  fee_type_id tinyint NOT NULL AUTO_INCREMENT,
  name varchar(30) NOT NULL,
  description varchar(140) NOT NULL,
  PRIMARY KEY(fee_type_id)
);

CREATE TABLE fees(
  fee_id int NOT NULL AUTO_INCREMENT,
  fee_type_id tinyint NOT NULL,
  name varchar(30) NOT NULL,
  description varchar(140) NOT NULL,
  due_date timestamp NOT NULL,
  fee float NOT NULL,
  FOREIGN KEY(fee_type_id) REFERENCES fee_types(fee_type_id),
  PRIMARY KEY(fee_id)
);

CREATE TABLE debtor_fees(
  debtor_fee_id int NOT NULL AUTO_INCREMENT,
  debtor_id int NOT NULL,
  fee_id int NOT NULL,
  additional_info varchar(140),
  paid boolean NOT NULL,
  FOREIGN KEY(debtor_id) REFERENCES users(user_id),
  FOREIGN KEY(fee_id) REFERENCES fees(fee_id),
  PRIMARY KEY(debtor_fee_id)
);