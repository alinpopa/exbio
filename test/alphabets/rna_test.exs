defmodule ExBio.Alphabets.RnaTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alphabets.Rna

  describe "alphabet" do
    test "creates new default alphabet reference" do
      {:ok, alphabet} = Rna.alphabet()
      assert is_reference(alphabet) == true
    end

    test "creates new iupac alphabet reference" do
      {:ok, alphabet} = Rna.alphabet(:iupac)
      assert is_reference(alphabet) == true
    end

    test "creates new n alphabet reference" do
      {:ok, alphabet} = Rna.alphabet(:n)
      assert is_reference(alphabet) == true
    end
  end

  describe "complement" do
    test "return the complement of the given character" do
      complement = Rna.complement(?a)
      assert complement == {:ok, ?u}

      complement = Rna.complement(?u)
      assert complement == {:ok, ?a}

      complement = Rna.complement(?U)
      assert complement == {:ok, ?A}

      complement = Rna.complement(?C)
      assert complement == {:ok, ?G}
    end
  end

  describe "revcomp" do
    test "return reverse complement of the given text" do
      complement = Rna.revcomp("acgun")

      assert complement == {:ok, "nacgu"}
    end
  end
end
