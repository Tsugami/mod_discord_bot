-- Add migration script here

ALTER TABLE "voice_state_update" DROP COLUMN "type";

DROP INDEX "voice_state_update_user_idx";

DROP TYPE "voice_state_update_type";

CREATE INDEX
    "voice_state_update_user_guild_idx" ON "voice_state_update" ("user_id", "guild_id");