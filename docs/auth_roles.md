# Authentication and Authorization Roles

## Roles and Permissions

### Administrator
- **Roles**: `directory.read`, `directory.write`, `directory.pii.read`
- **Permissions**: Full access to all resources.

### Manager
- **Roles**: `directory.read`, `directory.write`
- **Permissions**: Read and write access, excluding sensitive data.

### Analyst
- **Roles**: `directory.read`
- **Permissions**: Read-only access.

### Developer (Dev Mode)
- **Roles**: `directory.read`, `directory.write`, `directory.pii.read`
- **Permissions**: Full access (development only).

## Initial Users

| Username  | Password   | Roles                                |
|-----------|------------|--------------------------------------|
| admin     | admin123   | directory.read, directory.write, directory.pii.read |
| manager   | manager123 | directory.read, directory.write      |
| analyst   | analyst123 | directory.read                       |
| dev       | dev123     | directory.read, directory.write, directory.pii.read (Dev Mode) |
