defmodule ExBio.Alignment.Pairwise.AlignerTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alignment.Pairwise.MatchFunc
  alias ExBio.Alignment.Pairwise.Aligner
  alias ExBio.Alignment.Pairwise.Scoring
  require Scoring

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

  describe "with_capacity" do
    test "validates arguments" do
      eq = MatchFunc.eq(-1, 1)
      assert Aligner.with_capacity(0, 0, 3, 1, eq) == {:error, :invalid_args}
      assert Aligner.with_capacity(0, 0, -1, 1, eq) == {:error, :invalid_args}
      assert Aligner.with_capacity(0, 0, 3, -1, eq) == {:error, :invalid_args}
      assert Aligner.with_capacity(-1, 0, -1, -1, eq) == {:error, :invalid_args}
      assert Aligner.with_capacity(0, -1, -1, -1, eq) == {:error, :invalid_args}
    end

    test "creates aligner" do
      eq = MatchFunc.eq(-1, 1)
      {:ok, aligner} = Aligner.with_capacity(3, 1, -3, 0, eq)
      assert is_reference(aligner)
    end
  end

  describe "with scoring" do
    test "creates aligner" do
      eq = MatchFunc.eq(1, -1)
      {:ok, scoring} = Scoring.new(-5, -1, eq)
      {:ok, scoring} = Scoring.xclip(scoring, Scoring.min_score())
      {:ok, scoring} = Scoring.yclip(scoring, 0)
      {:ok, aligner} = Aligner.with_scoring(scoring)
      assert is_reference(aligner)
    end
  end

  describe "custom" do
    test "returns the right alignment" do
      eq = MatchFunc.eq(1, -1)
      {:ok, scoring} = Scoring.new(-5, -1, eq)
      {:ok, scoring} = Scoring.xclip(scoring, Scoring.min_score())
      {:ok, scoring} = Scoring.yclip(scoring, 0)
      {:ok, aligner} = Aligner.with_scoring(scoring)
      {:ok, alignment} = Aligner.custom(aligner, "ACCGTGGAT", "AAAAACCGTTGAT")

      assert %ExBio.Types.Alignment{} = alignment
      assert alignment.xstart == 0
      assert alignment.ystart == 4

      assert alignment.operations == [
               yclip: 4,
               match: 0,
               match: 0,
               match: 0,
               match: 0,
               match: 0,
               subst: 0,
               match: 0,
               match: 0,
               match: 0
             ]
    end
  end
end
