-- Add migration script here

CREATE TYPE voice_state_update_type AS ENUM ('JOIN', 'LEAVE', 'SWITCH');

CREATE TABLE
    "voice_state_update" (
        id SERIAL PRIMARY KEY,
        channel_id TEXT NOT NULL,
        guild_id TEXT NOT NULL,
        user_id TEXT NOT NULL,
        type voice_state_update_type NOT NULL,
        created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE UNIQUE INDEX "voice_state_update_user_idx" ON "voice_state_update" ("user_id")