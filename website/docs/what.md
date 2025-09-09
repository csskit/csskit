---
layout: markdown-base
permalink: /docs/
---

# What is csskit?

csskit is a program aimed at developers to make writing CSS more fun, to make
them more productive, and to give them ways to write better CSS. It's a
"toolchain", which a fancy way of saying it's a collection of different tools
bundled into one:

- It includes a "linter"; it helps find mistakes in your CSS and tells you how
  to correct them. For example if you misspell a selector or property it can
  tell you that.
- It includes a "formatter"; it will keep your code well organised using a
  standard code style, if you want to enforce tabs over spaces, a certain quote
  style, or a certain color syntax then csskit can do that for you!
- It's a "minifier"; you can write your CSS using generous whitespace and
  comments and use all the expressive syntax you want, and run the file through
  `csskit min` to produce a compressed version optimised for delivery on the
  web.
- It's a "transpiler"; you can write your CSS using new web standards, or new
  selectors, and csskit will try to convert it down to make sure your css works
  in all the browsers you want it to.
- It's a "bundler"; give it a project of CSS file and it'll smoosh them down
  into a set of smaller optimised files.
- It's an "analyzer"; you can ask it queries about your CSS, for example it can
  summarise all of the colours it finds in your CSS files and tell you the ideal
  colours to compliment them, or tell you which features you're using that might
  have limited browser support.
- It's an "LSP"; it comes ready to integrate directly into your IDE with plugins
  for [Zed](https://zed.dev/) and [VSCode](https://code.visualstudio.com/), it
  can add these kinds of improvements directly into your editor.

# Philosophy

The philosophy behind csskit is to be "css' missing toolchain". If you've ever
used a programming language like Rust, or Golang or Deno you might find it they
share some really nice ergonomic features in that each of these tools provides
you with almost everything you need to maintain a project in that language
without you having to reach out into the ecosystem. Running tests in a Rust
project is invariably `cargo test`, in Go: `go test`. Formatting your projects
code in Rust is as simple as `cargo fmt`, in Go: `go fmt`. To build your
project... well you get the idea.

Compare this to the JavaScript ecosystem, where there are many tools to solve
many problems, and often times tools have delicate configuration and a
combination of dependencies that make it difficult to move from project to
project. If you step into a JavaScript project and want to run tests, you may
have to discover if the project is using Jest or Vitest or Mocha or some other
testing framework, and running it might require Yarn or pnpm or npm or some
other tool. This provides a real battier to entry for many developers when going
from project to project, and teams can be held up switching between tools or
dedicating time to "configuring" these tools.

csskit wants to bring the ergonomic wins of the Rust & Goland toolchains to CSS,
an area that has been underinvested in for a long time. We have many excellent
tools in the CSS ecosystem but it often suffers the same fate as the JavaScript
ecosystem - a fragmented landscape and difficult to configure tools means CSS is
often harder to push to production than it needs to be.

# How does it compare to X?

There are many tools that might do similar things to csskit. It's worth noting
that right now csskit is still very new and it would be a _terrible_ choice to
replace any of your existing tooling with csskit. It's just experimental for
now, but one day it won't be. When that day comes you might be asking yourself
if you should use it over another tool or library. This hopes to provide some
information to help you:

## [postcss](https://postcss.org/)

[postcss](https://postcss.org/) is a wonderful tool designed to get developers
using the latest proposals from CSS specifications. It could be thought of as a
"minifier" and/or "transpiler". It is an excellent tool in the CSS ecosystem
and has continued to be essential for the last decade of its existence.

postcss is useful for the majority of developers, as it can translate modern CSS
into CSS compatible with older browsers. It's also essential for developers
who'd like to be more adventurous and try out new proposals or new and
interesting syntaxes. For example postcss has the ability to write nested CSS
long before it was a standard part of the language.

postcss' focus is on transpiling, so it doesn't have the ability to lint CSS.
While the plugin system offers maximum flexibility, it does come at a cost for
users who have to spend time configuring postcss to their needs.

csskit's goal is to eventually do everything postcss can for _most_ people's
needs. That is to say: csskit will always build on emerging specifications, but
it will avoid stepping into more experimental waters. If a browser supports some
kind of CSS syntax you can be sure that csskit will. If you're a more
adventurous developer looking to use experimental syntaxes that no browser
supports then continue using postcss!

If you're an ultra adventurous dev and you're looking to write your own plugins
to help you dream up of brand new ways to write CSS, then postcss is likely
going to be the better choice. It's written in JavaScript, and has a JavaScript
API while csskit is written in Rust and doesn't aim to have _any_ API for
extensions.

If you're looking for a battle tested production ready way to compress your CSS
then postcss is the answer. If you're looking for something with less
configuration or something a little faster then lightningcss might be a good
choice for you.

## [lightningcss](https://lightningcss.dev/)

[lightningcss](https://lightningcss.dev/) is a blazing fast css "minifier" /
"transpiler" that has gained a lot of traction in recent years, and has made a
huge impact in the CSS ecosystem. It's the tool that powers the
[Parcel](https://parceljs.org/)'s css minifier, and it has a standalone version.

It doesn't support the more experimental features that postcss does, but for
_most_ developers lightningcss would be a great choice over postcss - not least
because of how _quickly_ it can minify and transpile css!

lightningcss also leans on the shoulders of giants, it uses the fantastic
[cssparser](https://crates.io/crates/cssparser) library which also powers the
CSS engine Stylo - used in both [Servo](https://github.com/servo/servo/) and
[Firefox](https://www.firefox.com/).

Both lightningcss and csskit are written in Rust. That makes it _easier_ to
write _fast_ code compared to tools written in, say, JavaScript (like postcss).
lightningcss is currently the _fastest_ css minifier available to use, and it
can crunch down an average css file in just a few milliseconds. csskit a bit
slower, but still within a close range of lightningcss. Both lightningcss and
csskit are hundreds of times faster than postcss, if speed is your bag.

Comparing lightnigntcss to csskit, we can see how each project has made
different choices that change this dynamic, on a technical level:

- lightningcss is trying to do one thing well: minify css. The way it parses CSS
  means it _discards_ a lot of information that is unimportant when your output
  file is smaller CSS, for example it won't retain the case of keywords, or the
  exact way you wrote a color function. This is great for speed and skipping
  this extra info doesn't impact the resulting css output, but it does make it
  come out a lot faster.
- csskit aims to integrate with your IDE, and have flexible output formats, and
  so the parser needs to be resilient. csskit's "AST" tries to preserve source.
  This desire for a resilient parser is also why csskit doesn't use the
  `cssparser` crate that Servo, Firefox and lightningcss all do: they can simply
  discard CSS that they don't recognise, but csskit needs to retain it. Because
  of this the "AST Nodes" need to store more information, which takes more time
  to process, which means more memory and slower run times. Not too much slower
  though! Both will crunch css in the blink of an eye but if you're comparing
  numbers on a graph lightningcss will probably always win.

So if you're interested in raw speed, where every millisecond counts,
lightningcss is going to be the better choice. lightningcss also offers a
JavaScript API, so if you're interested in writing plugins to transform CSS and
postcss isn't your top choice then lightningcss would be one to look at more
closely.

Right now, at least, it looks as though lightningcss has no plans for formatting
or linting css. So if you're interested in a tool that offers a more complete
package then csskit may one day provide.

### [prettier](https://prettier.io/)

Prettier is an "opinionated code formatter" and is an industry staple. It works
well for lots of different languages, and css is no exception. If you're using
prettier to format other files in your project there's a good chance you'll want
to keep using it for css.

csskit aim is to format css code faster and "more prettier" than Prettier. Due
to csskit's inherant knowledge of css, at a deeper level than a tool like
Prettier it can make better choices about where to add line breaks and spaces.
Right now given csskit's experimental nature that's simply not true today, but
one day csskit might be a worthy choice to make over prettier.

### [stylelint](https://stylelint.io/)

Stylelint is a mighty CSS linter that comes with a whole slew of plugins to help
validate that you're writing clean code. It's effectively the _only_ choice to
make if you want to lint your css.

Stylelint's JavaScript API makes it nice an extensible, but like many JavaScript
projects it can slow down with a lot of plugins or if it needs to run over a lot
of CSS.

csskit's aim is to include a fully capable linter that will find the same kinds
of errors Stylelint does today. There are no plans to introduce a JavaScript
API, but it will have some manner of extending for custom lint rules. So while
today Stylelint is the right choice to make, that may change in the future.
