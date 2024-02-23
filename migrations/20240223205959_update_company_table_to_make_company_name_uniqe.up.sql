-- Add up migration script here
ALTER TABLE companies 
ADD CONSTRAINT unique_company_name UNIQUE (company_name); 
