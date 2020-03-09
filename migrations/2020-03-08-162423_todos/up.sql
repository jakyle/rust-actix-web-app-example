-- Your SQL goes here-- Your SQL goes here
CREATE TABLE todos(
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   finished BOOLEAN DEFAULT false,
   item VARCHAR(100) NOT NULL,
   description VARCHAR(250)
)