pub fn spawn_app() {
    let server = multiplexer::run().expect("Failed to bind address.");
    // Launch the server as a background task.
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence no variable binding.
    tokio::spawn(server);
}
