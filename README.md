# fibBot

This is a bot written in Rust that scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results. The action will support two parameters which a "enable_fib" (flag to enable Fibonacci calculation) and "max_threshold" (a threshold limit).

---  

## Functionality  
It uses a set of different functions that enable it to extract numbers from a PR made to the actual repository, calculate the fibonacci numbers of each of them and write the result as a comment under the PR.

---

## Prerequisites
- Install Rust
- Install Docker

---

## How to Use

### Add it to Your Workflow
```yml
  steps:
    - name: fibBot
      uses: @Guy-Ghis/fibBot@v1
      with: 
        enable_fib: true
        max_threshold: 100
        pr_number: ${{ github.event.pull_request.number }}
```
 

### Run Locally
To run the action locally, clone the repository:  

```shell
git clone https://github.com/Guy-Ghis/fibBot.git
```
Build the Rust project:

```shell
cargo build --release
```
Build Docker image
```shell
docker build -t fibBot .
```

#### Build and Run Using Docker  
Run the following command:  
```shell
docker run --rm \
  -e GITHUB_TOKEN=your_token \
  -e GITHUB_REPOSITORY=owner/repo \
  -e PR_NUMBER=123 \
  fibbot --enable-fib --max-threshold 100
```
