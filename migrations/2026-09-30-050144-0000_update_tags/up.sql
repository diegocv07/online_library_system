ALTER TABLE books DROP COLUMN tags;
ALTER TABLE books ADD COLUMN tags text[] NOT NULL DEFAULT ARRAY['None'];
-- Source - https://stackoverflow.com/a/469553
-- Posted by Baishampayan Ghose, modified by community. See post 'Timeline' for change history
-- Retrieved 2026-09-29, License - CC BY-SA 2.5

ALTER TABLE books ADD CONSTRAINT unique_isbn UNIQUE (isbn);

