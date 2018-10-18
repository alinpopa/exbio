defmodule ExBio.MixProject do
  use Mix.Project

  def project do
    [
      app: :exbio,
      version: "0.1.0",
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers,
      rustler_crates: rustler_crates()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.18.0"}
    ]
  end

  defp rustler_crates do
    [exbio: [
      path: "native/exbio",
      mode: (if Mix.env == :prod, do: :release, else: :debug),
    ],
    alignment: [
      path: "native/alignment",
      mode: (if Mix.env == :prod, do: :release, else: :debug),
    ],
    types: [
      path: "native/types",
      mode: (if Mix.env == :prod, do: :release, else: :debug),
    ]]
  end
end
