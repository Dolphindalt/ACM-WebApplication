CREATE TABLE event_types (
    event_type_id tinyint NOT NULL AUTO_INCREMENT,
    name varchar(40),
    description varchar(140),
    PRIMARY KEY(event_type_id)
);

INSERT INTO event_types (name, description)
VALUES ('Meeting', 'General club meetings for all members');

INSERT INTO event_types (name, description)
VALUES ('LAN Party', 'LAN Party, games, and pizza');

INSERT INTO event_types (name, description)
VALUES ('Programming Comp', 'Programming competitions');

INSERT INTO event_types (name, description)
VALUES ('Programming Prac', 'Practice for competitions');

INSERT INTO event_types (name, description)
VALUES ('Industry Trip', 'Going out and seeing companies');

INSERT INTO event_types (name, description)
VALUES ('Homecoming', 'Attendance for Homecoming events');

INSERT INTO event_types (name, description)
VALUES ('Club Rush', 'Club Rush help');

INSERT INTO event_types (name, description)
VALUES ('ASMT Meeting', 'Attendance for an ASMT proposal meeting');

INSERT INTO event_types (name, description)
VALUES ('DnD Night', 'Dungeons and Dragons game night');

INSERT INTO event_types (name, description)
VALUES ('Board Meeting', 'Board Meeting for Officers');

INSERT INTO event_types (name, description)
VALUES ('Study Group', 'Time for students to help each other with homework');

CREATE TABLE events (
    event_id int NOT NULL AUTO_INCREMENT,
    coordinator_id int,
    event_type_id tinyint NOT NULL,
    name varchar(30) NOT NULL,
    additional_info text,
    location varchar(50) NOT NULL,
    event_time timestamp NOT NULL,
    FOREIGN KEY(coordinator_id) REFERENCES users(user_id),
    FOREIGN KEY(event_type_id) REFERENCES event_types(event_type_id),
    PRIMARY KEY(event_id)
);

INSERT INTO events (coordinator_id, event_type_id, name, additional_info, location, event_time)
VALUES (1, 2, "LAN Party Test", "A test of the LAN party event!", "Museum lab", "2007-04-05T14:30:30");