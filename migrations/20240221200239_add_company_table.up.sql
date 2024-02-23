-- Add up migration script here 
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE companies (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  company_image_url TEXT,
  company_name TEXT NOT NULL,
  category TEXT,
  location TEXT,
  year_created TEXT,
  social_link TEXT,
  website_link TEXT,
  about TEXT,
  industry TEXT,
  employee_size INTEGER,
  heads_office_address TEXT,
  specialization TEXT,
  company_other_images TEXT[]
)
