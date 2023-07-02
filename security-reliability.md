# Web Application Security

- Message authentication: HMAC for message authentication to verify that the
  query parameters have been set by our API and that they haven ot been altered
  by a third party. This guarantees that the message has not been modified in
  transit and it allows you to verify the identity of the sender (data origin
  authentication).
    - Add HMAC tag to protect query parameters.
- Session fixation attacks (pg.490)
- Require user to provide current password as part of changing their password.
    - This prevents an attacker who managed to acquire a valid session token
      from locking the legitimate user out of their account.

# Web Application Reliability

- A `Typed` interface to `Session` (pg.491)
- Idempotency of requests.
