# ExBio

Bioinformatics library https://github.com/rust-bio/rust-bio Elixir bindings

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `exbio` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:exbio, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/exbio](https://hexdocs.pm/exbio).

## Prerequisites

- `rust >= 1.29.0`
- `elixir >= 1.7`

## Build/Tests

```
make
```

## Docker

- run the build/tests within docker by `make -C docker`

## Examples

```
iex -S mix

iex(1)> ExBio.PatternMatching.Bom.bom("ACGGCTAGGAAAAAGACTGAGGACTGAAAA", "GAAAA")
{:ok, [8, 25]}
```
