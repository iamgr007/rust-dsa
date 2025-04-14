# Rust DSA Repository

Welcome to our Rust Data Structures & Algorithms (DSA) repository!  
Here, we solve classic and advanced DSA problems in Rust, learn from each other, and maintain high code quality.

---

## 📚 Problem Roadmap

Check off problems as you solve them!  
Feel free to add more problems or suggest new topics.

### Arrays & Strings
- [ ] Two Sum
- [ ] Best Time to Buy and Sell Stock
- [ ] Product of Array Except Self
- [ ] Longest Substring Without Repeating Characters

### Linked Lists
- [ ] Reverse Linked List
- [ ] Merge Two Sorted Lists
- [ ] Linked List Cycle

### Stacks & Queues
- [ ] Valid Parentheses
- [ ] Min Stack
- [ ] Implement Queue using Stacks

### Trees
- [ ] Binary Tree Inorder Traversal
- [ ] Maximum Depth of Binary Tree
- [ ] Serialize and Deserialize Binary Tree

### Graphs
- [ ] Number of Islands
- [ ] Clone Graph
- [ ] Course Schedule (Topological Sort)
- [ ] Dijkstra’s Algorithm

### Dynamic Programming
- [ ] Climbing Stairs
- [ ] Coin Change
- [ ] Longest Increasing Subsequence
- [ ] 0/1 Knapsack

### Sorting & Searching
- [ ] Merge Sort
- [ ] Quick Sort
- [ ] Binary Search

### Heaps & Priority Queues
- [ ] Kth Largest Element in an Array
- [ ] Merge K Sorted Lists

### Advanced Rust Concepts
- [ ] Generational Arenas for Graph Nodes
- [ ] Benchmarking with Criterion.rs
- [ ] Concurrent Data Structures (Mutex, Arc)



---

## 🗂️ Repository Structure

    rust-dsa/
    ├── data_structures/
    │ ├── stack/
    │ ├── queue/
    │ ├── linked_list/
    │ └── tree/
    ├── algorithms/
    │ ├── sorting/
    │ ├── searching/
    │ └── dynamic_programming/
    ├── problems/
    │ ├── easy/
    │ ├── medium/
    │ └── hard/
    ├── tests/
    │ └── integration/
    ├── .github/
    │ ├── workflows/
    │ └── ISSUE_TEMPLATE/
    ├── .gitignore
    ├── Cargo.toml
    └── README.md


---

## 🚀 Getting Started

1. **Create a new branch for your solution:**

        git checkout -b feat/<your-username>/<problem-name>
        Example: feat/bagati/two-sum

2. **Install Rust:**  
[Install Rust](https://www.rust-lang.org/tools/install) if you haven’t already.

3. **Run tests:**

       cargo test


---

## 🤝 Collaboration Workflow

1. **Create a new branch for your solution:**

        git checkout -b feat/<your-username>/<problem-name>
        Example: feat/bagati/two-sum

2. **Add your solution in the appropriate folder.**

3. **Write tests for your code!**

4. **Commit with a descriptive message:**
        
        git add .
        git commit -m "feat(array): add two sum solution and tests"


5. **Push your branch:**

        git push origin feat/<your-username>/<problem-name>


6. **Open a Pull Request (PR):**
- Go to GitHub and open a PR to `main`.
- Tag friends for review.
- Address feedback and make improvements.

---

## 🛡️ Code Quality & CI

- All code must pass [Clippy](https://github.com/rust-lang/rust-clippy) lints and tests.
- PRs require at least 2 approvals before merging.
- CI runs automatically on every PR.

---

## 📝 Contribution Guidelines

- **Atomic commits:** One logical change per commit.
- **Descriptive PRs:** Explain your approach and reasoning.
- **Respect code reviews:** Learn from each other!
- **No raw pointers in graph code:** Use indices or safe abstractions.

---

## 📅 Maintenance & Progress

- **Weekly sync:** Review solutions, assign new problems, and update the checklist.
- **Task tracking:** Use GitHub Projects or Issues for assignments and progress.

---

## 📢 Invite Your Friends!

- Share the repo link.
- Add collaborators in GitHub settings.
- Let’s learn Rust DSA together!

---

**Happy coding! 🚀**

---

**References:**  
- [Rust Book](https://doc.rust-lang.org/book/)  
- [LeetCode](https://leetcode.com/problemset/all/)  
- [GeeksforGeeks DSA](https://www.geeksforgeeks.org/data-structures/)  
- [Rustlings](https://github.com/rust-lang/rustlings)

---

**Tip:**  
For graph implementations, prefer index-based approaches over raw pointers for safety and idiomatic Rust.


