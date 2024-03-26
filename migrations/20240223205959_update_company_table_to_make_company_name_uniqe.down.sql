-- Add down migration script here
ALTER TABLE companies
DROP CONSTRAINT IF EXIST unique_company_name;
