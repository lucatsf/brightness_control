# Brightness Control

Brightness Control é um aplicativo para controlar o brilho do monitor em sistemas Linux, escrito em Rust usando GTK. Ele permite que os usuários ajustem facilmente o brilho de seus monitores externos diretamente de uma interface gráfica.

## Funcionalidades

- Ajuste de brilho utilizando uma barra deslizante.
- Suporte a múltiplos monitores com seleção de monitor através de um combobox.

## Capturas de Tela

(Adicione aqui capturas de tela do seu aplicativo)

## Requisitos

- Rust e Cargo
- GTK 3
- `ddcutil`

## Instalação

### Ubuntu/Debian

1. Instale as dependências necessárias:

    ```bash
    sudo apt update
    sudo apt install libgtk-3-dev ddcutil
    ```

2. Baixe o pacote `.deb` da seção de releases do GitHub.

3. Instale o pacote `.deb`:

    ```bash
    sudo dpkg -i brightness_control_0.1.0_amd64.deb
    ```

## Manualmente

1. Certifique-se de ter Rust e Cargo instalados:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone o repositório:

```bash
git clone git@github.com:lucatsf/brightness_control.git
cd brightness_control
```

3. Compile o projeto:

```bash
cargo build --release
```

4. Torne o binário executável:
    
```bash
chmod +x target/release/brightness_control
```

5. Mova o binário para um diretório no seu PATH:

```bash
sudo mv target/release/brightness_control /usr/local/bin/
```

## Uso

1. Execute o aplicativo:

```bash
sudo brightness_control
```
O uso de sudo é necessário para acessar os controles de brilho do monitor via ddcutil.

2. Utilize a interface gráfica para selecionar o monitor e ajustar o brilho.

## Contribuição

Contribuições são bem-vindas! Siga os passos abaixo para contribuir:

1. Faça um fork do repositório.
2. Crie uma branch para a sua feature (`git checkout -b feature/nome-da-feature`).
3. Faça commit das suas mudanças (`git commit -am 'Adiciona uma nova feature'`).
4. Faça push para a branch (`git push origin feature/nome-da-feature`).
5. Crie um novo Pull Request.

## Licença

Este projeto está licenciado sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.

## Agradecimentos

- [ddcutil](https://www.ddcutil.com/): por fornecer a ferramenta para controle de brilho via DDC/CI.
- [GTK-rs](https://gtk-rs.org/): por permitir a integração com GTK em Rust.