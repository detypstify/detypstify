#import "@preview/tablex:0.0.8": cellx, hlinex, tablex
#import "/neurips2023.typ": *
#import "/logo.typ": LaTeX, LaTeXe, TeX

#let affls = (
  airi: ("AIRI", "Moscow", "Russia"),
  skoltech: (
    department: "AI Center",
    institution: "Skoltech",
    location: "Moscow",
    country: "Russia"),
  skoltech2: (
    department: "AI Center",
    institution: "Skoltech",
    location: "Moscow",
    country: "Russia"),
)

#let authors = (
  (name: "Justin Restivo",
   // affl: "Yale",
   email: "justin.restivo@yale.edu",
   equal: true),
  (name: "Jachym Putta",
   // affl: "Yale",
   email: "jachym.putta.@yale.edu",
   equal: true),
)

#show: neurips2023.with(
  title: "Formatting Instructions For NeurIPS 2023",
  authors: (authors, affls),
  keywords: ("Machine Learning", "NeurIPS"),
  // one paragraph only
  abstract: [
    Something about im2latex, openai, OCR
  ],
  bibliography: bibliography("main.bib"),
  bibliography-opts: (title: none, full: true),  // Only for example paper.
  accepted: false,
)

= Introduction and Related Work
  == OpenAI Im2Latex problem

= System Design
  == Model
  == Machine Learning
  ==
  == Algorithm

== Implementation
  == Model
    === Dataset
    === Training
  == Webapp
    == ONNX
    == Burn
    == Wasm + WGPU
= Results
  == Model
  == Webapp



