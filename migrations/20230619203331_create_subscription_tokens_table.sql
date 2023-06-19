-- For creating a new table, a gradual update is unnecessary. Simply add the
-- new table in a migration while the currently running application ignores it.
-- You should never deploy the migration script and a modification to the app
-- logic that accompanies it simultaneously. Deploy only the table-creating
-- migration without deploying the app. Afterward, deploy the modified app.
-- Incompatibility issues should not arise, as the previous app would not be
-- using the newly created table in the first place.
CREATE TABLE
    subscription_tokens (
        subscription_token TEXT NOT NULL,
        subscriber_id uuid NOT NULL REFERENCES subscriptions (id),
        PRIMARY KEY (subscription_token)
    );
