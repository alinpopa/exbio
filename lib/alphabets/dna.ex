defmodule ExBio.Alphabets.Dna do
  alias ExBio.Nif.RustBio

  def alphabet(),
    do: RustBio.alph_dna_alphabet()
end
