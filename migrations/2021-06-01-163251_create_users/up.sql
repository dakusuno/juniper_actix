-- Your SQL goes here**

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users(

   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   name_all VARCHAR(255) NOT NULL,
   email VARCHAR(255) UNIQUE NOT NULL

)