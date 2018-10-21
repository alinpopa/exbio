defmodule ExBio.Alignment.Distance do
  alias ExBio.Nif.RustBio

  def hamming(x, y), do: RustBio.align_dist_hdist(x, y)

  def levenshtein(x, y), do: RustBio.align_dist_ldist(x, y)
end
