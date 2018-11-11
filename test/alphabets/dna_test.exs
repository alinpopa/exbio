defmodule ExBio.Alphabets.DnaTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alphabets.Dna

  describe "alphabet" do
    test "creates new alphabet reference" do
      {:ok, alphabet} = Dna.alphabet()
      assert is_reference(alphabet) == true
    end
  end

  describe "complement" do
    test "return the complement of the given character" do
      complement = Dna.complement(?a)
      assert complement == ?t

      complement = Dna.complement(?t)
      assert complement == ?a

      complement = Dna.complement(?A)
      assert complement == ?T

      complement = Dna.complement(?T)
      assert complement == ?A
    end
  end
end
