defmodule ExBio.Alphabets.Dna do
  alias ExBio.Nif.RustBio

  def alphabet(type \\ :default)

  def alphabet(type) when type in [:default, :iupac, :n],
    do: RustBio.alph_dna_alphabet(type)

  def complement(char),
    do: RustBio.alph_dna_complement(char)

  def revcomp(text),
    do: RustBio.alph_dna_revcomp(text)
end
