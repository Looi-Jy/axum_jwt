# Rust JWT authentication API example

You may need to setup database and RSA key pair to make this example fully work.
1. Setup database which can run with `sqlx` library, using **PostgresQL** on this example.
2. Generate **RSA** key pair (private & public key) and put them into `src/auth/key` folder.
