---
layout: markdown-base
permalink: /docs/
---

# ❓ What is csskit?

csskit is a program aimed at developers to make writing CSS more fun, to make
them more productive, and to give them ways to write better CSS. It's a
"toolchain", which a fancy way of saying it's a collection of different tools
bundled into one:

- <b>💅</b> <a href="/docs/fmt">Formatting</a>; it will keep your code well
  organised using a standard code style, if you want to enforce tabs over
  spaces, a certain quote style, or a certain color syntax then csskit can do
  that for you!
- <b>🔍</b> <a href="/docs/lint">Linting</a>; it helps find mistakes in your
  CSS and tells you how to correct them. For example if you misspell a selector
  or property it can tell you that.
- <b>⚡</b> <a href="/docs/min">Minifier</a>; you can write your CSS using
  generous whitespace and comments and use all the expressive syntax you want,
  and run the file through `csskit min` to produce a compressed version
  optimised for delivery on the web.
- <b>🔄</b> <a href="/docs/transform">Transpiler</a>; you can write your CSS
  using new web standards, or new selectors, and csskit will try to convert it
  down to make sure your css works in all the browsers you want it to.
- <b>📦</b> <a href="/docs/bundle">Bundler</a>; give it a project of CSS file
  and it'll smoosh them down into a set of smaller optimised files.
- <b>🔬</b> <a href="/docs/analyze">Analyzer</a>; you can ask it queries about
  your CSS, for example it can summarise all of the colours it finds in your CSS
  files and tell you the ideal colours to compliment them, or tell you which
  features you're using that might have limited browser support.
- <b>💻</b> <a href="/docs/lsp">LSP</a>; it comes ready to integrate directly
  into your IDE with plugins for <a href="/docs/lsp/zed">Zed</a> and
  <a href="/docs/lsp/vscode">VSCode</a>, it can provide guidance and completion
  directly into your editor.

## Philosophy

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

## How does it compare?

> [!WARNING]
> csskit is still alpha software! It is ready to experiment with and provide
> feedback but using it for production projects might result in bugs!
>
> When making the choice between very new alpha software or battle tested well
> used software for production code, always pick the more reliable option!

### ⚡ 🔄 [postcss](https://postcss.org/) & [cssnano](https://cssnano.github.io/cssnano/)

[postcss](https://postcss.org/) is a wonderful tool with more than a decade of
iteration. [cssnano](https://cssnano.github.io/cssnano/) builds on the success
of postcss with a low-config option ready to convert & minify your CSS.

Both tools focus on transforming (transpiling & minifying) CSS, so they don't
have the ability to lint CSS. While the plugin system offers maximum
flexibility, it does come at a cost for users who have to spend time configuring
postcss to their needs.

For more adventurous developers postcss has a plugin system so you can write
your own transformations. This keeps postcss ahead of the curve when it comes to
experimental features. The plugin system uses JavaScript so it will likely be
familiar.

csskit's goal is to eventually do everything postcss can for _most_ people's
needs. That is to say: csskit will always build CSS that browsers support. If
you interested in more experimental features that no browser supports then
postcss or cssnano will probably always be the best options for that.

If you're looking for a battle tested production ready way to compress your CSS
then postcss is the answer. If you're looking for something with less
configuration or something a little faster then lightningcss might be a good
choice for you.

So pick postcss or cssnano if:

- <b>🪨</b> You want something rock solid and well tested.
- <b>🧪</b> You want to build on top of more experimental features before the
  browsers even get them.
- <b>🐌</b> You don't mind waiting a few seconds for your css to build.

### ⚡ 🔄 📦 [lightningcss](https://lightningcss.dev/) or [Parcel](https://parceljs.org/)

[lightningcss](https://lightningcss.dev/) is a blazing fast css "minifier" /
"transpiler" that aims to be a replacement tool for cssnano or postcss. It's the
tool that powers the [Parcel](https://parceljs.org/)'s css minifier.

It doesn't support the more experimental features that postcss does, but for
_most_ developers lightningcss would be a great choice over postcss - not least
because of how _quickly_ it can minify and transpile css! lightningcss is
written in _Rust_ (just like csskit) and builds on top of the excellent
[cssparser](https://crates.io/crates/cssparser) library which also powers the
both [Servo](https://github.com/servo/servo/) and
[Firefox](https://www.firefox.com/) browsers!

csskit is roughly on par with the speed of lightningcss. csskit is still very
new and so it doesn't minify code as well, and by the time it does it might be
a little slower than lightningcss. Both lightningcss and csskit are hundreds of
times faster than postcss.

<details>
  <summary>Some numbers and details</summary>

Remember that it's a little bit silly to benchmark a well tested and well
developed tool such as lightningcss with a tool which is still very new like
csskit. Additionally different files will perform differently, not just due to
the size of the CSS but also the features used. The point being every file is
different.

That being said some people like to see the numbers. So here is a benchmark
comparing both tools trying to minify Tailwind 2.2.19 - roughly 3mb of CSS:

```
$ hyperfine --warmup 10 -N100 \
  "csskit min coverage/popular/tailwind.2.2.19.min.css" \
  "lightningcss --bundle -m coverage/popular/tailwind.2.2.19.min.css"

Benchmark 1: csskit min coverage/popular/tailwind.2.2.19.min.css
  Time (mean ± σ):     172.5 ms ±   1.6 ms    [User: 155.2 ms, System: 15.8 ms]
  Range (min … max):   169.2 ms … 179.0 ms    100 runs

Benchmark 2: lightningcss --bundle -m coverage/popular/tailwind.2.2.19.min.css
  Time (mean ± σ):     174.4 ms ±   3.3 ms    [User: 128.7 ms, System: 50.8 ms]
  Range (min … max):   167.0 ms … 186.5 ms    100 runs

Summary
  csskit ran 1.01 ± 0.02 times faster than lightningcss

$ csskit min coverage/popular/tailwind.2.2.19.min.css | wc -c
  3317759

$ lightningcss --bundle -m coverage/popular/tailwind.2.2.19.min.css
  2860287

$ lightningcss --version
  lightningcss 1.0.0-alpha.66

$ csskit --version
  0.0.1 # ccbba4dc6e090757e443fbdc582b556aa88ca6b
```

Both take almost exactly the same amount of time - around 175ms give or take.

Comparing lightnigntcss to csskit on a technical level, we can look at some
details around the different choices that change this dynamic:

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

</details>

So pick lightningcss or parcel if:

- <b>🚄</b> You want best in class performance.
- <b>💁</b> You're less interested in the other features, like formatting or
  IDE integration.

#### 💅 [prettier](https://prettier.io/)

[Prettier](https://prettier.io/) is an "opinionated code formatter" and an
industry staple. It works well for lots of different languages, and CSS is no
exception. If you're using prettier to format other files in your project
there's a good chance you'll want to keep using it for css.

csskit aim is to format css code faster and "more prettier" than Prettier. Due
to csskit's inherant knowledge of css, at a deeper level than a tool like
Prettier it can make better choices about where to add line breaks and spaces.
Right now given csskit's experimental nature that's simply not true today, but
one day csskit might be a worthy choice to make over prettier.

So pick prettier if:

- <b>🔨</b> You're already using it to format your css, and have no issues with
  the output it gives you.

#### 🔍 [stylelint](https://stylelint.io/)

Stylelint is a mighty CSS linter that comes with a whole slew of plugins to help
validate that you're writing clean code. It's effectively the _only_ choice to
make if you want to lint your CSS. Stylelint's JavaScript API makes an
extensible choice too.

csskit's aim is to include a fully capable linter that will find the same kinds
of errors Stylelint does today. There are no plans to introduce a JavaScript
API, but it will have some manner of extending for custom lint rules. So while
today Stylelint is the right choice to make, that may change in the future.

- <b>🔨</b> You're already using it to lint your code.
- <b>🧪</b> You want to build a set of custom lint rules using a JavaScript API.
- <b>🐌</b> You don't mind waiting a few seconds for it to run.
