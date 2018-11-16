[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
Invoke-WebRequest -OutFile rustup-init.exe https://win.rustup.rs/
.\rustup-init -y --default-toolchain stable
