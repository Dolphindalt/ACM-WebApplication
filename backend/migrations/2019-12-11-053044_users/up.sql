CREATE TABLE passwords (
  password_id int NOT NULL AUTO_INCREMENT,
  password varchar(255) NOT NULL,
  verification_code varchar(8),
  PRIMARY KEY(password_id)
);

CREATE TABLE user_types (
  user_type_id tinyint NOT NULL AUTO_INCREMENT,
  name varchar(20) NOT NULL,
  description varchar(50),
  PRIMARY KEY(user_type_id)
);

CREATE TABLE users(
    user_id int NOT NULL AUTO_INCREMENT,
    password_id int NOT NULL,
    user_type tinyint NOT NULL,
    first_name varchar(20) NOT NULL,
    last_name varchar(20) NOT NULL,
    email varchar(30) NOT NULL UNIQUE,
    FOREIGN KEY(password_id) REFERENCES passwords(password_id),
    FOREIGN KEY(user_type) REFERENCES user_types(user_type_id),
    PRIMARY KEY(user_id)
);