# Retirement Calculator

- Browser-based retirement calculator written in Rust and Yew

## Demonstration

- https://www.croftsoft.com/portfolio/retirement-calculator/

## Usage

- cd retirement-calculator/
- cargo install trunk
- trunk serve
- http://localhost:8080/

## Deploy

- trunk build --release --public-url my-folder/retirement-calculator
- rm ../my-website-project/public_html/my-folder/retirement-calculator/*
- cp dist/* ../my-website-project/public_html/my-folder/retirement-calculator/

## History

- Initial release: 2022-07-16
- This is my first application written using the Yew framework
  - https://yew.rs/
- I converted it from a Java servlet that I wrote back in 1999
  - https://sourceforge.net/p/croftsoft/code/HEAD/tree/trunk/apps/src/com/croftsoft/apps/retirement/RetirementServlet.java
