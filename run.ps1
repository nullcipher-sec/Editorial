# run.ps1 - Sultana del Lago Editores (Windows PowerShell)
[CmdletBinding()]
param (
    [Parameter(Position = 0)]
    [string]$Action = "help"
)

$ErrorActionPreference = "Continue"

$RootDir = $PSScriptRoot
$ComposeFile = Join-Path $RootDir "deploy\docker-compose.dev.yml"
$EnvFile = Join-Path $RootDir "deploy\.env"
$EnvExample = Join-Path $RootDir "deploy\.env.dev.example"

# 1. Validar que Docker este disponible
if (-not (Get-Command "docker" -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: Docker no esta instalado o no esta disponible en PATH." -ForegroundColor Red
    exit 1
}

# 2. Copiar deploy/.env si no existe
if (-not (Test-Path $EnvFile)) {
    Write-Host "Creando deploy\.env desde deploy\.env.dev.example..." -ForegroundColor Cyan
    Copy-Item $EnvExample $EnvFile
}

function Invoke-Compose {
    docker compose --env-file $EnvFile -f $ComposeFile @args
}

function Wait-ForCore {
    Write-Host "Esperando que el Core responda en http://localhost:8080/actuator/health..." -ForegroundColor Yellow
    for ($i = 1; $i -le 60; $i++) {
        try {
            $resp = Invoke-RestMethod -Uri "http://localhost:8080/actuator/health" -TimeoutSec 2 -ErrorAction SilentlyContinue
            if ($resp -and $resp.status -eq "UP") {
                Write-Host "El Core esta listo!" -ForegroundColor Green
                return $true
            }
        } catch {
            # Esperar a que el backend este disponible
        }
        Start-Sleep -Seconds 1
    }

    Write-Host "ERROR: el Core no respondio en http://localhost:8080/actuator/health." -ForegroundColor Red
    Invoke-Compose logs --tail 80 editorial_core
    return $false
}

function Start-Services {
    Invoke-Compose up -d --build postgres ldap editorial_core noticiero
    if (-not (Wait-ForCore)) {
        exit 1
    }
}

function Ensure-Services {
    docker compose --env-file $EnvFile -f $ComposeFile up -d --no-build postgres ldap editorial_core noticiero 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        Invoke-Compose up -d --build postgres ldap editorial_core noticiero
    }
    if (-not (Wait-ForCore)) {
        exit 1
    }
}

function Run-Tui {
    param([string]$ServiceName)
    Ensure-Services
    Invoke-Compose build $ServiceName
    Invoke-Compose run --rm --no-deps $ServiceName
}

function Stop-Services {
    $project = "editorial-dev"
    $containers = docker ps -q --filter "label=com.docker.compose.project=$project"
    if ($containers) {
        $containers | ForEach-Object {
            docker stop $_ 2>&1 | Out-Null
            docker rm -f $_ 2>&1 | Out-Null
        }
    }
    Invoke-Compose --profile "*" down --remove-orphans
}

function Clean-Services {
    $project = "editorial-dev"
    $containers = docker ps -q --filter "label=com.docker.compose.project=$project"
    if ($containers) {
        $containers | ForEach-Object {
            docker stop $_ 2>&1 | Out-Null
            docker rm -f $_ 2>&1 | Out-Null
        }
    }
    Invoke-Compose --profile "*" down --remove-orphans -v
}

switch ($Action.ToLower()) {
    "services" {
        Start-Services
    }
    "stop" {
        Stop-Services
    }
    "clean" {
        Clean-Services
    }
    "reset" {
        Clean-Services
    }
    "tui_recluta" {
        Run-Tui "tui_reclutamiento"
    }
    { $_ -in @("tui_atencion", "tui_produccion", "tui_rrhh", "tui_reclutamiento", "tui_eventos") } {
        Run-Tui $Action.ToLower()
    }
    default {
        Write-Host "Uso:"
        Write-Host "  .\run.ps1 services"
        Write-Host "  .\run.ps1 stop"
        Write-Host "  .\run.ps1 clean"
        Write-Host "  .\run.ps1 tui_atencion"
        Write-Host "  .\run.ps1 tui_produccion"
        Write-Host "  .\run.ps1 tui_rrhh"
        Write-Host "  .\run.ps1 tui_recluta"
        Write-Host "  .\run.ps1 tui_eventos"
        exit 1
    }
}
