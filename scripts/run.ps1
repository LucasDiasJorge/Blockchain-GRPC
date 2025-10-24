param(
    [switch]$Proxy,        # Start only the HTTP proxy
    [switch]$NoRebuild,    # Don't trigger a build if binaries are missing
    [switch]$Release,      # Prefer release binary
    [switch]$Wait          # Wait for started processes in foreground
)

<#
  scripts/run.ps1

  Execução conveniente dos binários já compilados (não recompila se os binários existirem).

  Uso:
    # Inicia ambos (gRPC + proxy) se existirem
    .\run.ps1

    # Inicia só o proxy
    .\run.ps1 -Proxy

    # Não tenta compilar caso os binários não existam (falha com mensagem)
    .\run.ps1 -NoRebuild

    # Força a preferência por binários em target/release
    .\run.ps1 -Release

    # Fica aguardando os processos (útil para debugar em uma janela)
    .\run.ps1 -Wait

  Estratégia:
  - Procura o binário em target/(release|debug)/<name>.exe
  - Se não encontrado e -NoRebuild não foi passado, executa `cargo build` (release se -Release)
  - Inicia os processos com Start-Process e redireciona stdout/stderr para logs em .\logs\
#>

Set-StrictMode -Version Latest

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$repoRoot = Resolve-Path "$scriptDir\.."

Push-Location $repoRoot

if (-not (Test-Path -Path "logs")) { New-Item -ItemType Directory -Path "logs" | Out-Null }

function Get-BinaryPath([string]$name, [bool]$preferRelease) {
    $cfgs = @()
    if ($preferRelease) { $cfgs += "release"; $cfgs += "debug" } else { $cfgs += "debug"; $cfgs += "release" }
    foreach ($cfg in $cfgs) {
        $exe = "target\$cfg\$name.exe"
        if (Test-Path $exe) { return (Resolve-Path $exe).Path }
    }
    return $null
}

function Ensure-Binary([string]$name, [bool]$preferRelease, [bool]$noRebuild) {
    $path = Get-BinaryPath -name $name -preferRelease:$preferRelease
    if ($null -ne $path) { return $path }

    if ($noRebuild) {
        Write-Error "Binary '$name' not found in target/(debug|release). Use cargo build or remove -NoRebuild."; exit 1
    }

    # Trigger incremental build only if necessary
    if ($preferRelease) {
        Write-Host "Binary '$name' not found. Running 'cargo build --release' (may take a while) ..."
        & cargo build --release
    } else {
        Write-Host "Binary '$name' not found. Running 'cargo build' (may take a while) ..."
        & cargo build
    }

    $path = Get-BinaryPath -name $name -preferRelease:$preferRelease
    if ($null -eq $path) { Write-Error "After build, binary '$name' still not found."; exit 1 }
    return $path
}

# Nome dos binários conforme Cargo.toml
$mainBin = "blockchain-grpc"
$proxyBin = "http_proxy"

$preferRelease = [bool]$Release

if ($Proxy) {
    $startGrpc = $false
    $startProxy = $true
} else {
    $startGrpc = $true
    $startProxy = $true
}

$processes = @()

if ($startGrpc) {
    $grpcPath = Ensure-Binary -name $mainBin -preferRelease:$preferRelease -noRebuild:$NoRebuild
    $grpcLog = "logs\grpc.log"
    Write-Host "Starting gRPC server: $grpcPath"
    $p = Start-Process -FilePath $grpcPath -ArgumentList @() -RedirectStandardOutput $grpcLog -RedirectStandardError $grpcLog -PassThru
    Write-Host "gRPC PID: $($p.Id)  (log: $grpcLog)"
    $processes += @{ Name = 'grpc'; Proc = $p; Log = $grpcLog }
}

if ($startProxy) {
    $proxyPath = Ensure-Binary -name $proxyBin -preferRelease:$preferRelease -noRebuild:$NoRebuild
    $proxyLog = "logs\proxy.log"
    Write-Host "Starting HTTP proxy: $proxyPath"
    $p2 = Start-Process -FilePath $proxyPath -ArgumentList @() -RedirectStandardOutput $proxyLog -RedirectStandardError $proxyLog -PassThru
    Write-Host "Proxy PID: $($p2.Id)  (log: $proxyLog)"
    $processes += @{ Name = 'proxy'; Proc = $p2; Log = $proxyLog }
}

if ($Wait) {
    Write-Host "Waiting for processes. Press Ctrl+C to stop. Tailing logs..."
    try {
        foreach ($entry in $processes) {
            Write-Host "--- ${($entry.Name).ToUpper()} LOG (${ $entry.Log }) ---"
            Start-Job -ScriptBlock { param($f) Get-Content -Path $f -Wait -Tail 10 } -ArgumentList $entry.Log | Out-Null
        }
        while ($true) { Start-Sleep -Seconds 1 }
    } finally {
        Write-Host "Cleanup: stopping started processes"
        foreach ($entry in $processes) {
            try { Stop-Process -Id $entry.Proc.Id -ErrorAction SilentlyContinue } catch {}
        }
    }
} else {
    Write-Host "Started processes in background. Use Get-Process to inspect or tail logs in .\logs\"
}

Pop-Location
