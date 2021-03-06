defmodule ExBio.Alphabets.DnaTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alphabets.Dna

  describe "alphabet" do
    test "creates new default alphabet reference" do
      {:ok, alphabet} = Dna.alphabet()
      assert is_reference(alphabet) == true
    end

    test "creates new iupac alphabet reference" do
      {:ok, alphabet} = Dna.alphabet(:iupac)
      assert is_reference(alphabet) == true
    end

    test "creates new n alphabet reference" do
      {:ok, alphabet} = Dna.alphabet(:n)
      assert is_reference(alphabet) == true
    end
  end

  describe "complement" do
    test "return the complement of the given character" do
      complement = Dna.complement(?a)
      assert complement == {:ok, ?t}

      complement = Dna.complement(?t)
      assert complement == {:ok, ?a}

      complement = Dna.complement(?A)
      assert complement == {:ok, ?T}

      complement = Dna.complement(?T)
      assert complement == {:ok, ?A}
    end
  end

  describe "revcomp" do
    test "return reverse complement of the given text" do
      complement = Dna.revcomp("AACCTT")

      assert complement == {:ok, "AAGGTT"}
    end
  end
end
