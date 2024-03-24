-- Add down migration script here

-- Drop the trigger
DROP TRIGGER IF EXISTS update_job_postings_updated_at ON job_postings;

-- Drop the function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop the table
DROP TABLE IF EXISTS job_postings;

