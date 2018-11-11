defmodule ExBio.Alphabets.Dna do
  alias ExBio.Nif.RustBio

  def alphabet(),
    do: RustBio.alph_dna_alphabet()

  def complement(char),
    do: RustBio.alph_dna_complement(char)
end
