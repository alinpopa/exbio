defmodule ExBio.Alignment.Pairwise.AlignerTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alignment.Pairwise.MatchFunc
  alias ExBio.Alignment.Pairwise.Aligner

  describe "new" do
    test "validates arguments" do
      eq = MatchFunc.eq(1, 0)
      assert Aligner.new(3, 1, eq) == {:error, :invalid_args}
      assert Aligner.new(-1, 1, eq) == {:error, :invalid_args}
      assert Aligner.new(3, -1, eq) == {:error, :invalid_args}
    end

    test "creates aligner" do
      eq = MatchFunc.eq(1, 0)
      {:ok, aligner} = Aligner.new(-3, 0, eq)
      assert is_reference(aligner)
    end
  end
end
