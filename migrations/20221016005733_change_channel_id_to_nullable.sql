-- Add migration script here

ALTER TABLE
    "voice_state_update"
ALTER COLUMN
    "old_channel_id" DROP NOT NULL;