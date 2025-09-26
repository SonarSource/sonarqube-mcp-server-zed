# SonarQube MCP Server Zed Extension

A SonarQube MCP Server extension for Zed, running inside a Docker container.

## Installation

* When using SonarQube Cloud:

```json
{
  "sonarqube_token": "YOUR_SONARQUBE_TOKEN",
  "sonarqube_org": "SONARQUBE_ORGANIZATION_KEY",
  "docker_path": "DOCKER_PATH"
}
```

* When using SonarQube Server:

```json
{
  "sonarqube_token": "YOUR_SONARQUBE_USER_TOKEN",
  "sonarqube_url": "YOUR_SONARQUBE_SERVER_URL",
  "docker_path": "DOCKER_PATH"
}
```

The `docker_path` is the path to a docker executable. Examples:
  - Linux/macOS: `/usr/bin/docker` or `/usr/local/bin/docker`
  - Windows: `C:\Program Files\Docker\Docker\resources\bin\docker.exe`

## Build

Run the following command to build the extension:

`cargo build`
