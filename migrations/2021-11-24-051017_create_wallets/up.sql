-- Your SQL goes here
CREATE TABLE currency (
  name VARCHAR(3) PRIMARY KEY
);

INSERT INTO currency VALUES
  ('JPY'), 
  ('USD'); 

CREATE TABLE wallet (
  id VARCHAR(64) PRIMARY KEY,
  deposit BIGINT NOT NULL,
  currency VARCHAR(3),
  foreign key (currency) references currency(name)
);

