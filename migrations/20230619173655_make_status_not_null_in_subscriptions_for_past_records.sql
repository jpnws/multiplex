-- We wrap the whole migration in a transaction to make sure it succeeds or
-- fails atomically. This is called a SQL transaction, and `sqlx` does not do it
-- automatically. Understand why we're doing all this in the first place:
-- We aim to update our subscriptions service with a new NOT NULL database
-- field `status`. Assume we draft a migration to add this field, alter the
-- insert logic in the app, and deploy as-is. The database and application
-- deployments occur at differing speeds. Consequently, users might experience
-- errors during the subscription process amidst the deployment. If the database
-- gets deployed first, users will face errors until the app is fully deployed.
-- This is due to the old app inserting new subscribers without the `status`
-- field data, which is NOT NULL. Conversely, if the app is deployed first,
-- users will encounter errors, as the app tries to insert data into the
-- non-existent `subscribers.status` field in the old database. A pragmatic
-- solution involves first deploying the DB migration, making the
-- `subscriptions.status` field nullable. Subsequently, adjust the app to
-- include logic for the `status` field and deploy it. Once deployed, construct
-- another migration to backfill past subscriber entries with non-null values
-- and set `status` to NOT NULL. Remember to execute the migration query within
-- a transaction to terminate if any operation fails.
BEGIN;

-- Backfill `status` for past entries.
UPDATE subscriptions
SET
    status = 'confirmed'
WHERE
    status IS NULL;

-- Make `status` mandatory
ALTER TABLE subscriptions
ALTER COLUMN status
SET
    NOT NULL;

COMMIT;
