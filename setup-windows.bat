@echo off
setlocal
title Lipsa Agent Skills Bundle - setup
set "REPO=https://github.com/lipsapriyadarsinee101-oss/Lipsaagent-skills-bundle.git"
set "DIR=%USERPROFILE%\Lipsaagent-skills-bundle"

where git >nul 2>&1 || (echo Git is not installed. Get it from https://git-scm.com/download/win & goto :end)
where cargo >nul 2>&1 || (echo Rust is not installed. Get it from https://rust-lang.org/tools/install & goto :end)

if exist "%DIR%\.git" (
    echo Updating %DIR% ...
    cd /d "%DIR%" || goto :end
    git pull || goto :end
    git submodule update --init --depth 1 || goto :end
    cargo run --release -q -- uninstall --agent all >nul
) else (
    echo Downloading the bundle into %DIR% ...
    git clone --recurse-submodules --shallow-submodules "%REPO%" "%DIR%" || goto :end
    cd /d "%DIR%" || goto :end
)

echo.
echo Installing skills ...
cargo run --release -- install --agent all || goto :end
echo.
cargo run --release -q -- doctor
echo.
echo Done. Restart Codex or your Agent Skills host to load the skills.

:end
echo.
pause
