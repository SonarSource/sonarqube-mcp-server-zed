/*
 * SonarQube MCP Server Zed Extension
 * Copyright (C) 2025 SonarSource
 * mailto:info AT sonarsource DOT com
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the Sonar Source-Available License Version 1, as published by SonarSource SA.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the Sonar Source-Available License for more details.
 *
 * You should have received a copy of the Sonar Source-Available License
 * along with this program; if not, see https://sonarsource.com/license/ssal/
 */
use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result};

const DOCKER_IMAGE: &str = "mcp/sonarqube";

#[derive(Debug, Deserialize, JsonSchema)]
struct SonarQubeContextServerSettings {
    sonarqube_token: String,
    sonarqube_url: Option<String>,
    sonarqube_org: Option<String>,
    docker_path: String
}

struct SonarQubeModelContextExtension;

impl zed::Extension for SonarQubeModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("mcp-server-sonarqube", _project)?;
        let Some(settings) = settings.settings else {
            return Err("Missing SonarQube settings".into());
        };
        let settings: SonarQubeContextServerSettings = serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: settings.docker_path,
            args: vec![
                "run".to_string(),
                "-i".to_string(),
                "--rm".to_string(),
                "-e".to_string(),
                "SONARQUBE_TOKEN".to_string(),
                "-e".to_string(),
                "SONARQUBE_URL".to_string(),
                "-e".to_string(),
                "SONARQUBE_ORG".to_string(),
                DOCKER_IMAGE.to_string()
            ],
            env: vec![
                ("SONARQUBE_TOKEN".into(), settings.sonarqube_token),
                ("SONARQUBE_URL".into(), settings.sonarqube_url.unwrap_or("".to_string())),
                ("SONARQUBE_ORG".into(), settings.sonarqube_org.unwrap_or("".to_string()))
            ],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions = include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema = serde_json::to_string(&schemars::schema_for!(SonarQubeContextServerSettings)).map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(SonarQubeModelContextExtension);
