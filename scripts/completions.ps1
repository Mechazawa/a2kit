$PowerShellProfile = $PROFILE.CurrentUserAllHosts

function Install-Completions {
    $CompletionFile = "./completions/_a2kit.ps1"
    $ProfileContent = Get-Content -Path $PowerShellProfile

    if (-not ($ProfileContent -contains ". $CompletionFile")) {
        Add-Content -Path $PowerShellProfile -Value ". $CompletionFile"
        Write-Host "Completions installed"
    } else {
        Write-Host "Completions already installed"
    }
}

function Remove-Completions {
    $CompletionFile = "./completions/_a2kit.ps1"
    $ProfileContent = Get-Content -Path $PowerShellProfile

    if ($ProfileContent -contains ". $CompletionFile") {
        $ProfileContent = $ProfileContent | Where-Object { $_ -ne ". $CompletionFile" }
        Set-Content -Path $PowerShellProfile -Value $ProfileContent
        Write-Host "Completions removed"
    } else {
        Write-Host "Completions not found"
    }
}

param (
    [string]$Action
)

switch ($Action) {
    "install" { Install-Completions }
    "remove" { Remove-Completions }
    default { Write-Host "Usage: script.ps1 {install|remove}" }
}
