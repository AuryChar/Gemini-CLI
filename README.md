# Gemini-CLI(Rust)

This is an experimental gemini client written in rust.

to use it, follow the steps below.

## Clone the repository
```bash
git clone https://github.com/AuryChar/Gemini-CLI.git
```

## Enter the directory

```bash
cd Gemini-CLI

```

## Set environment variable

Create a file named `.env` in the root directory of the project and add the following line:
```bash
touch .env
```

Enter in `.env` file with `nano`(or any other text editor):
```bash
nano .env
```

Inside the file, add the following line:
```bash
GOOGLE_API_KEY='YOUR_GOOGLE_API_KEY'
```

## Build the project

```bash
cargo build --release
```

## Install the binary

```bash
cargo install --path .
```

Now you can run typing "Gemini-CLI":
```bash
Gemini-CLI
```
