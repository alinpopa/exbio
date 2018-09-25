defmodule ExBio.PatternMatching.Bom do
  alias ExBio.Nif.RustBio

  def bom(text, pattern), do: RustBio.pm_bom(text, pattern)
end
