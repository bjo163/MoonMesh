# Security Policy

MoonMesh is a security-sensitive networking project. Do not disclose private keys, credentials, or unpatched exploit details in public issues.

## Supported line

Only the default development branch is currently supported while the project is pre-release.

## Reporting

Use GitHub's private security reporting mechanism for vulnerabilities when available. Include the affected component, version/commit, impact, reproducibility and a minimal safe proof. Do not include live credentials or private key material.

## Security principles

- No custom cryptographic primitives.
- Authenticate peers by cryptographic identity.
- Keep privileged OS networking code isolated.
- Treat all network input as hostile.
- Enforce replay, length, TTL and hop limits at protocol boundaries.
- Do not claim production readiness before end-to-end security tests pass.
