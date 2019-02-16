defmodule ExBio.MixProject do
  use Mix.Project

  def project do
    [
      app: :exbio,
      version: "0.1.0",
      elixir: "~> 1.8",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers,
      rustler_crates: rustler_crates(),
      dialyzer: [ flags: ["-Wunmatched_returns", :error_handling, :race_conditions, :underspecs]]
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:dialyxir, "~> 1.0.0-rc.4", only: [:dev], runtime: false},
      {:rustler, "~> 0.19.0"}
    ]
  end

  defp rustler_crates do
    [exbio: [
      path: "native/exbio",
      mode: (if Mix.env == :prod, do: :release, else: :debug),
    ]]
  end
end
