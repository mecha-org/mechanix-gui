# Testing the Mechanix Configuration Server

This directory contains unit tests for the Mechanix Configuration Server. The tests are organized by module:

- `validator_test.rs`: Tests for the validator module
- `database_test.rs`: Tests for the database module
- `conf_interface_test.rs`: Tests for the configuration interface module
- `client_test.rs`: Tests for the client module (Note: These tests require a D-Bus session to be running)

## Running the Tests

To run all tests, use the following command from the project root:

```bash
cargo test
```

To run tests for a specific module, use:

```bash
cargo test --test <test_file_name_without_extension>
```

For example:

```bash
cargo test --test validator_test
```

## Test Dependencies

The tests use the following dependencies:

- `tempfile`: For creating temporary directories and files
- `mockall`: For creating mock objects
- `serial_test`: For running tests serially when needed

## Testing Approach

### Validator Tests

The validator tests create TOML schemas in memory and validate them against the validation rules. They test both valid and invalid schemas, as well as validation of settings against schemas.

### Database Tests

The database tests create a temporary directory for the database and test the database operations. They use the `tempfile` crate to create temporary directories and the `serial_test` crate to ensure tests run serially.

### Configuration Interface Tests

The configuration interface tests create a temporary directory for the database and test the D-Bus interface. They use the `tempfile` crate for temporary directories and `tokio` for async testing.

### Client Tests

The client tests attempt to test the client's interaction with the D-Bus interface. However, these tests require a D-Bus session to be running, which might not be available in all environments. In a real-world scenario, we might want to mock the D-Bus connection for more isolated testing.

## Notes on Testing D-Bus Interfaces

Testing D-Bus interfaces can be challenging because they require a running D-Bus session. In a CI/CD environment, you might need to set up a D-Bus session before running the tests. Alternatively, you could mock the D-Bus connection, but this would require significant changes to the codebase.

For the client tests, we've taken a simple approach that requires a D-Bus session to be running. If you're running the tests locally, make sure you have a D-Bus session running. If you're running the tests in a CI/CD environment, you might need to set up a D-Bus session or skip these tests.