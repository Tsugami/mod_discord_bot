-- Add migration script here

ALTER TABLE voice_state_update
ALTER COLUMN
    channel_id DROP NOT NULL;