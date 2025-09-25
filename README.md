# SonarQube MCP Server Zed Extension

A SonarQube MCP Server extension for Zed. It runs within a Docker container.

When using SonarQube Cloud:

```json
{
  "sonarqube_token": "YOUR_SONARQUBE_TOKEN",
  "sonarqube_org": "SONARQUBE_ORGANIZATION_KEY",
  "docker_path": "DOCKER_PATH"
}
```

When using SonarQube Server:

```json
{
  "sonarqube_token": "YOUR_SONARQUBE_USER_TOKEN",
  "sonarqube_url": "YOUR_SONARQUBE_SERVER_URL",
  "docker_path": "DOCKER_PATH"
}
```

- `docker_path`: Path to docker executable. Examples:
  - Linux/macOS: `"/usr/bin/docker"` or `"/usr/local/bin/docker"`
  - Windows: `"C:\Program Files\Docker\Docker\resources\bin\docker.exe"`
