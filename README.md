Find-in-files for Python Projects (in Rust)
===========================================

* Finds text string from Python source files (implemented in Rust)

* Ignores following directories by default: ".pytype", "site-packages", "__pycache__", ".mypy_cache", "downloads", "uploads", "tmp", ".git", "migrations"

* "--include-site-packages" option can be used to include "site-packages" directory as well


This is a test project writing software in a new (to me) language without reading any tutorial or any documentation :) https://chatgpt.com/share/1f316838-3a4a-4002-8ef6-6cf804f24f6d


Install
-------

        make


Run
---

        find-py (text to search)

        Options:
    
            --include-site-packages

