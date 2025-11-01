# Script otimizado para build incremental rápido

Write-Host "🚀 Build Incremental Otimizado" -ForegroundColor Cyan

# Verifica se sccache está instalado
if (Get-Command sccache -ErrorAction SilentlyContinue) {
    Write-Host "✓ sccache detectado - usando cache" -ForegroundColor Green
    $env:RUSTC_WRAPPER = "sccache"
} else {
    Write-Host "⚠ sccache não instalado. Instale com:" -ForegroundColor Yellow
    Write-Host "  cargo install sccache" -ForegroundColor Yellow
}

# Build incremental
Write-Host "`n📦 Compilando..." -ForegroundColor Cyan
cargo build --release

# Mostra estatísticas do cache se disponível
if (Get-Command sccache -ErrorAction SilentlyContinue) {
    Write-Host "`n📊 Estatísticas do Cache:" -ForegroundColor Cyan
    sccache --show-stats
}
