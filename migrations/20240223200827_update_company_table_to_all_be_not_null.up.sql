-- Add up migration script here
ALTER TABLE companies 
ALTER COLUMN company_image_url SET NOT NULL,
ALTER COLUMN company_name SET NOT NULL,
ALTER COLUMN category SET NOT NULL,
ALTER COLUMN location SET NOT NULL,
ALTER COLUMN year_created SET NOT NULL,
ALTER COLUMN social_link SET NOT NULL,
ALTER COLUMN website_link SET NOT NULL,
ALTER COLUMN about SET NOT NULL,
ALTER COLUMN industry SET NOT NULL,
ALTER COLUMN employee_size SET NOT NULL,
ALTER COLUMN heads_office_address SET NOT NULL,
ALTER COLUMN specialization SET NOT NULL,
ALTER COLUMN company_other_images SET NOT NULL;
