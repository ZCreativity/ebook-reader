# Problem Definition

An ebook reader is a GUI-based program that allows a user to open files containing books for viewing page by page, with the ability to keep track of the reading progress, change font size, display in single or double page mode, etc.

# Objectives

The primary objective is to develop a program for managing EPUB format books, using the Rust DRUID library (see link in references). Aim to customize the user interaction to the maximum extent possible, potentially leveraging other programs as a reference (both for basic functionality and for improving shortcomings). The code structure should extensively use tests to develop and verify individual functionalities. Additionally, allow the user to activate a "draft correction" mode that enables editing the content in the presence of errors (e.g., typos), generating a new file with each save. The second objective is to integrate the camera system with OCR (see link in reference) to merge possession of the paper book with the digital one in one of the following ways:

1. Jump from the paper book being read to the point reached by taking a photo of the page.
2. Perform the reverse, i.e., from two recognized pages, indicate which page corresponds to the text reached in the digital version.

# Contacts

Prof. Alessandro Savino

# References:

- DRUID: https://github.com/linebender/druid
- Public domain books (legal): https://www.gutenberg.org
- OCR methods with RUST: https://www.linkedin.com/pulse/ocr-rustleptess-tesseract-ha%C3%AFkel-ouaghrem/
