# Retirement Calculator

- Browser-based retirement calculator written in Rust and Yew

## Demonstration

- https://www.gamespawn.com/arcade/retirement-calculator/

## Usage

- cd retirement-calculator/
- cargo install trunk
- trunk serve
- http://localhost:8080/

## Deploy

- trunk build --release
- rm ../my_website_project/public_html/retirement_calculator/*
- cp dist/* ../my_website_project/public_html/retirement_calculator/

## History

- Initial release: 2022-07-16
- This is my first application written using the Yew framework
  - https://yew.rs/
- I converted it from a Java servlet that I wrote back in 1999
  - https://sourceforge.net/p/croftsoft/code/HEAD/tree/trunk/apps/src/com/croftsoft/apps/retirement/RetirementServlet.java
