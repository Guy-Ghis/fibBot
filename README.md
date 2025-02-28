# fibBot
# Project: "FibBot" GitHub Action

**Overview:**

Develop a GitHub Action in Rust that scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results. The action will support two parameters (e.g., a flag to enable Fibonacci calculation and a threshold limit).

Each day, ensure that you have a new working component or improvement that corresponds to the milestone(s) for that day.

---

### 6-Day Milestone Plan with Daily Deliverables

### **Day 1: Kickoff & Action Review**

- **Milestone 1:** **Review GitHub Actions Fundamentals**
    - **Deliverable:** A summary or checklist of key concepts on creating custom GitHub Actions, especially for non-JavaScript implementations.
- **Milestone 2:** **Project Initialization**
    - **Deliverable:** A new Rust project (e.g., created with `cargo new fibbot`) with an initial repository structure that includes the basic `action.yml` file.
- **End-of-Day Outcome:** You should have a repository set up with a clear understanding of how actions work and the metadata file in place.

### **Day 2: Minimal Action Implementation**

- **Milestone 1:** **Build a “Hello World” Action in Rust**
    - **Deliverable:** A Rust program that prints "Hello, world!" when executed.
- **Milestone 2:** **Test the Basic Action**
    - **Deliverable:** A GitHub workflow that triggers your action, with confirmation that the "Hello, world!" output appears in the action logs.
- **End-of-Day Outcome:** You’ll have a minimal working GitHub Action written in Rust that is successfully running on GitHub.

### **Day 3: Parameter Handling & Input Parsing**

- **Milestone 1:** **Define and Parse Inputs**
    - **Deliverable:** Update your `action.yml` to include two parameters (e.g., `enable_fib` and `max_threshold`) and implement input parsing in your Rust code.
- **Milestone 2:** **Local Testing of Parameters**
    - **Deliverable:** A demonstration (via logs or test cases) that the parameters are correctly read and validated.
- **End-of-Day Outcome:** By the end of Day 3, your action should correctly receive and process inputs from the GitHub workflow.

### **Day 4: Core Logic Development**

- **Milestone 1:** **Extract Numbers from PR Content**
    - **Deliverable:** A function that parses a sample string (simulating PR content) and extracts numerical values.
- **Milestone 2:** **Implement Fibonacci Calculator**
    - **Deliverable:** A robust Fibonacci function in Rust, with tests covering edge cases and efficiency.
- **End-of-Day Outcome:** You should have core logic for both number extraction and Fibonacci calculation, complete with test cases that confirm their functionality.

### **Day 5: Integration & GitHub API Interaction**

- **Milestone 1:** **Combine Parsing & Calculation**
    - **Deliverable:** Integrated functionality that extracts numbers and computes their Fibonacci values.
- **Milestone 2:** **Interact with GitHub API**
    - **Deliverable:** Rust code that posts a comment to a pull request using the GitHub API, integrating authentication and the computed results.
- **End-of-Day Outcome:** Your action should now process PR content, compute Fibonacci numbers for extracted values, and post a comment with the results.

### **Day 6: Full Workflow Testing & Documentation**

- **Milestone 1:** **End-to-End Workflow Testing**
    - **Deliverable:** A fully functioning GitHub Action tested in a dummy repository, with successful execution on pull requests.
- **Milestone 2:** **Documentation & Final Touches**
    - **Deliverable:** A comprehensive README with setup instructions, parameter configurations, and code comments; plus any necessary code refinements.
- **End-of-Day Outcome:** By the end of Day 6, you will have a polished, documented, and fully working GitHub Action that is ready for public use
