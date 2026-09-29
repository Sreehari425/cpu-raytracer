# CPU Ray Tracer

A community project to build CPU-based ray tracers following Peter Shirley’s **Ray Tracing in One Weekend** books, with everyone creating their own implementation.

## Table of Contents

- [About the project](#about-the-project)
- [Prerequisites](#prerequisites)
- [Syllabus](#syllabus)
- [Repository structure](#repository-structure)
- [How to contribute (step by step)](#how-to-contribute-step-by-step)
- [Rules and conventions](#rules-and-conventions)
- [Getting help](#getting-help)
- [License](#license)

---

## About the project

This is a community project where we build a CPU-based ray tracer together. You can use any programming language you like and follow along with Peter Shirley’s **[Ray Tracing in One Weekend](https://raytracing.github.io/)** books. Everyone will make their own version of the ray tracer in a separate folder. This lets us compare the results and see how the same ideas can be implemented in different programming languages. If someone gets stuck, we can help each other out and figure things out together.

Before we get started, it’s worth mentioning that this CPU ray tracer will be our first step into the world of graphics programming. This is intended to be a long-term learning project, so it’s best suited for anyone who is genuinely curious about the subject and willing to stick with it. With that said, I hope you’re all doing well, and welcome !

---

## Prerequisites

You don’t need any prior experience with graphics programming to get started, but it helps to be comfortable with a few basics beforehand.

### Maths

High-school-level maths should be more than enough. The most important topic to be familiar with is **linear algebra**, as we’ll be using it quite a bit throughout the project.

We’ll also be using **[Awesome Graphics](https://nira-community.github.io/awesome-graphics/)** as our wiki for graphics programming and getting started with the subject. Feel free to check it out before we begin if you want to brush up on the prerequisites or get a better idea of what this project is all about.

---

## Syllabus

Our main resource is the **[Ray Tracing Books](https://raytracing.github.io/)** series, which contains three books. We will go through them in this order:

1. [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
2. [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
3. [Ray Tracing: The Rest of Your Life](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)

We will finish each book chapter by chapter, and move on to the next book once the current one is done.

---

## Repository structure

```
cpu-raytracer/
├── README.md
├── CONTRIBUTING.md
├── .github/
│   └── CODEOWNERS         
└── implementations/
    ├── alice-rust/    
    ├── bob-cpp/
    └── carol-python/
```

### Inside your implementation folder

Your folder is named `implementations/<github-username>-<language>/`.

**The only required file is a `README.md` at the root of your folder.** Everything else is up to you and depends on your language and build system. Use whatever layout is normal for your ecosystem.

Your folder `README.md` should include:
- Language and version
- How to build and run
- A sample rendered image (keep it small)

Examples of what a folder might look like:

```
# C++ (headers need an include folder)
implementations/bob-cpp/
├── README.md
├── CMakeLists.txt
├── include/
│   ├── vec3.h
│   └── ray.h
├── src/
│   └── main.cpp
└── output/

# Rust (Cargo layout)
implementations/alice-rust/
├── README.md
├── Cargo.toml
└── src/
    └── main.rs

# Python
implementations/carol-python/
├── README.md
├── requirements.txt
└── raytracer/
    ├── vec3.py
    └── main.py
```

---

## How to contribute (step by step)

You **do not** need write access to this repository. You work in your own fork and open a pull request.

### 1. Fork the repository
Click **Fork** at the top right of this repo. This creates a copy under your GitHub account.

### 2. Clone your fork
```bash
git clone https://github.com/<your-username>/cpu-raytracer.git
cd cpu-raytracer
```

### 3. Add the original repo as `upstream` (to stay in sync)
```bash
git remote add upstream https://github.com/nira-community/cpu-raytracer
git remote -v
```

### 4. Create your branch
Branch name format: `<github-username>/<language>`

```bash
git checkout -b alice/rust
```

> Note: Git does not allow `[` or `]` in branch names, so use a slash like `alice/rust`.

### 5. Create your folder and start coding
```bash
mkdir -p implementations/alice-rust
# add your code and a README.md inside implementations/alice-rust/
# organise the inside however your language expects (src/, include/, etc.)
```

Work **only** inside your own folder `implementations/<username>-<language>/`.

### 6. Commit often
```bash
git add implementations/alice-rust
git commit -m "alice-rust: add vec3 class and PPM output"
```

Use meaningful messages, ideally with the chapter number.

### 7. Push to your fork
```bash
git push -u origin alice/rust
```

Later pushes are just `git push`.

### 8. Open a pull request
1. Go to your fork on GitHub and click **Compare & pull request**.
2. Base repository: `nira-community/cpu-raytracer`, base branch: `main`.
3. Title: `alice-rust: done anti-aliasing`
4. Describe what you completed and attach a rendered image if you can.

The maintainer will review and merge. You can keep pushing to the same branch to update the PR, or open a new PR for each milestone.

### 9. Keep your fork up to date
```bash
git fetch upstream
git checkout main
git merge upstream/main
git push origin main

# then bring updates into your branch
git checkout alice/rust
git merge main
```

---

## Rules and conventions

- Only edit files inside **your own** `implementations/<username>-<language>/` folder
- Do not modify other people's folders, the root README, or shared files (open an issue instead)
- Do not commit build artifacts (`target/`, `build/`, `node_modules/`, `__pycache__/`, binaries)
- Be kind. This is a learning project. All questions are welcome.

---

## Getting help

If you have any doubts or get stuck, open a **GitHub Issue** in this repo so we can discuss it and help each other. You can also join our **[Discord server](https://discord.gg/dnQ8qamtA4)** to discuss things in more detailed.

---

## License

This project is released under the **MIT License**, permitting use, copying, modification, merging, publishing, distribution, sublicensing, and sale of copies of the software, subject to the terms of the license.

